# StarryOS `futex` / `get_robust_list` / `set_robust_list` 调研

本文面向准备在 StarryOS 上补充或修复“用户态同步”相关 syscall 测试的开发者。

范围限定为这一组 syscall 及其直接关联路径：

- `futex`
- `get_robust_list`
- `set_robust_list`
- `set_tid_address`
- `clone(... CLONE_CHILD_CLEARTID ...)`
- 线程退出时的 `clear_child_tid` / `robust_list` 处理

这份文档的目标不是复述 Linux man page，而是回答三个更具体的问题：

1. StarryOS 现在到底实现到了哪一层。
2. 这些实现和哪些内核对象、退出路径、地址空间语义耦合。
3. 后面做测试或修复时，优先应该盯哪些风险点。

## 1. 仓库里已有的相关文档

这块目前没有一份专门只讲 `futex`/`robust_list` 的独立文档，但已经有一些分散材料：

- `docs/docs/design/architecture/starryos-internals.md`
  - 这是 StarryOS 总体内部机制文档。
  - 提到了 `ProcessData.futex_table`、`Thread.robust_list_head`，也明确把 futex 放进了 `task` 子系统。
  - 适合先建立大图景，再回来看本文件。

- `docs/docs/crates/starry-kernel.md`
  - 这是 `os/StarryOS/kernel` 这个 crate 的总览。
  - 会告诉你 futex 属于 `task` 路径，不是一个孤立 syscall。

- `docs/docs/crates/starry-vm.md`
  - 这个很重要。
  - `futex`、`clear_child_tid`、`robust_list` 都依赖 `VmPtr` / `VmMutPtr` 访问用户态地址。
  - 如果后面修 bug 涉及用户指针读写，这份文档值得一起看。

- `os/StarryOS/kernel/CHANGELOG.md`
  - 这里能看到和这一组直接相关的历史变更：
    - `process pending futex entry in exit_robust_list`
    - `fix futex wait user-memory access`
  - 它能帮助你判断哪些行为是最近补上的，哪些地方可能还比较脆。

- `os/StarryOS/docs/bug-fixes.md`
  - 不是 futex 专题文档，但展示了这个项目记录并发/语义问题的风格。
  - 后面如果你修出一个同步语义 bug，可以参考它的写法补记录。

- `test-suit/starryos/GUIDE.md`
  - 这是 StarryOS 测试套件组织方式文档。
  - 后面你要新增 futex/robust-list 测试时，目录、pipeline、`qemu-*.toml` 的组织都按这里来。

除此以外，当前仓库中唯一直接覆盖 futex 的现有测试是：

- `test-suit/starryos/normal/qemu-smp1/bugfix/bug-futex-wait-wake`

它是一个回归用例，不是完整的同步语义测试组。

## 2. 代码入口总览

### 2.1 syscall 分发入口

统一入口在：

- `os/StarryOS/kernel/src/syscall/mod.rs`

相关分发项：

- `Sysno::set_tid_address => sys_set_tid_address(...)`
- `Sysno::futex => sys_futex(...)`
- `Sysno::get_robust_list => sys_get_robust_list(...)`
- `Sysno::set_robust_list => sys_set_robust_list(...)`

这里的要点很简单：这一组 syscall 不是散落在不同子系统里，而是全部挂在 `task/sync` 相关路径上。

### 2.2 主要实现文件

和这组 syscall 最相关的文件如下：

- `os/StarryOS/kernel/src/syscall/sync/futex.rs`
  - syscall 层的 `sys_futex`、`sys_get_robust_list`、`sys_set_robust_list`

- `os/StarryOS/kernel/src/task/futex.rs`
  - futex 等待队列、futex key、futex table、本地/共享 futex 归类逻辑

- `os/StarryOS/kernel/src/task/ops.rs`
  - 线程退出时的 `clear_child_tid` 唤醒
  - `exit_robust_list()` 扫描与 owner-dead 处理

- `os/StarryOS/kernel/src/task/mod.rs`
  - `Thread` 中保存 `clear_child_tid` 与 `robust_list_head`
  - `ProcessData` 中保存 `futex_table`

- `os/StarryOS/kernel/src/syscall/task/thread.rs`
  - `sys_set_tid_address`

- `os/StarryOS/kernel/src/syscall/task/clone.rs`
  - `CLONE_CHILD_CLEARTID`、`CLONE_CHILD_SETTID` 路径

- `os/StarryOS/kernel/src/syscall/task/execve.rs`
  - `execve()` 后对 `clear_child_tid` 的处理

## 3. 当前实现结构

## 3.1 `Thread` 和 `ProcessData` 里保存了什么

在当前设计里：

- `Thread` 持有
  - `clear_child_tid: AtomicUsize`
  - `robust_list_head: AtomicUsize`

- `ProcessData` 持有
  - `futex_table: Arc<FutexTable>`

这反映了一个很重要的设计判断：

- `clear_child_tid` 和 `robust_list_head` 是线程私有状态。
- futex 等待队列表默认是进程级共享状态。

这个分层对后续测试很关键，因为它直接决定了：

- 同一进程内不同线程是否共享 futex 等待对象。
- `get_robust_list(tid, ...)` 为什么要按线程查。
- 线程退出时为什么既要处理 `clear_child_tid`，又要处理 robust list。

## 3.2 futex key 是怎么决定的

核心逻辑在 `task/futex.rs` 的 `FutexKey::new()`。

StarryOS 并不是只用“虚拟地址值”来区分 futex，而是先看这个地址落在哪类映射后端：

- 如果地址落在普通私有映射上，生成 `FutexKey::Private { address }`
- 如果地址落在共享内存后端 `Backend::Shared(...)` 上，生成 `FutexKey::Shared`
- 如果地址落在文件映射后端 `Backend::File(...)` 上，也生成 `FutexKey::Shared`

共享 key 不直接拿虚拟地址做全局标识，而是用：

- 共享页对象 `SharedPages` 的弱引用地址
- 或文件映射上的 `futex_handle`

再加上区域内偏移组成逻辑 identity。

这意味着当前实现已经考虑了一个比较硬核的问题：

- 同一物理共享对象被映射到不同虚拟地址时，waiter 仍应汇聚到同一个 futex 队列。

这也是后面做共享内存 futex 测试时最值得验证的点之一。

## 3.3 futex table 的分层

StarryOS 现在有两层 futex table：

- 进程私有 futex：
  - 直接放在 `current().as_thread().proc_data.futex_table`

- 共享 futex：
  - 放在全局 `SHARED_FUTEX_TABLES`
  - 按共享对象 identity 查找或创建对应 `FutexTable`

此外，`FutexGuard` 在 drop 时会清理空 entry：

- 如果 entry 的强引用计数足够低
- 且等待队列为空
- 就会把该 key 从表里移除

所以这不是一个“只增不减”的永久映射表。

## 3.4 wait queue 为什么不用 no-IRQ 自旋锁

`task/futex.rs` 里有一条很重要的注释：

> Futex waits must re-check the user value while serializing with wakeups.
> That re-check may fault and sleep, so this queue cannot use a no-IRQ spinlock.

翻成更直白的话就是：

- futex 等待前，内核要重新读用户态那个 futex word。
- 这次读取可能触发缺页、睡眠或其他 faultable 行为。
- 因此等待队列锁不能是“禁中断自旋锁 + 不允许睡眠”的那类锁。

当前实现使用的是 `ax_sync::Mutex<VecDeque<(Waker, u32)>>`。

这个设计直接对应了 `kernel/CHANGELOG.md` 里记录过的一次真实修复：

- “修复 futex 等待中的用户态内存访问”

也就是说，这一段不是纸面上的理论问题，而是这个项目已经踩过一次坑。

## 4. `sys_futex()` 当前支持什么

实现位于：

- `os/StarryOS/kernel/src/syscall/sync/futex.rs`

当前明确支持的 command 有：

- `FUTEX_WAIT`
- `FUTEX_WAIT_BITSET`
- `FUTEX_WAKE`
- `FUTEX_WAKE_BITSET`
- `FUTEX_REQUEUE`
- `FUTEX_CMP_REQUEUE`

其他 command 目前直接走：

- `Err(AxError::Unsupported)`

所以如果你的测试列表里想做：

- `FUTEX_LOCK_PI`
- `FUTEX_UNLOCK_PI`
- `FUTEX_FD`
- `FUTEX_WAKE_OP`

这些目前都不在实现范围内。

### 4.1 `WAIT` / `WAIT_BITSET`

等待路径大致是：

1. 先用 `uaddr.vm_read()?` 读用户态 futex word
2. 如果值和期望值不同，直接返回 `WouldBlock`
3. 解析 timeout
4. 获取或创建 futex entry
5. 进入 wait queue
6. 被唤醒后，如果 `owner_dead` 置位，则返回 `EOWNERDEAD`

这里有几个关键点。

#### 4.1.1 先做 fast path 比较

如果用户态值已经不等于期望值，内核不会真的入队等待，而是直接失败返回。

这个行为是 futex 最基本的语义之一，也是你后面最应该补的基础测试之一。

#### 4.1.2 timeout 语义不是同一种

`futex_wait_timeout()` 的实现里：

- `FUTEX_WAIT` 把传入 `timespec` 当作相对超时
- `FUTEX_WAIT_BITSET` 则把它当成绝对时间
  - 若带 `FUTEX_CLOCK_REALTIME` 就用 `wall_time()`
  - 否则用 `monotonic_time()`
  - 然后做差得到剩余时间

这和 Linux futex ABI 是对齐方向的，后面测试时不能把两者混在一起。

#### 4.1.3 owner-dead 由 wait 路径消费

如果线程退出时，内核通过 robust list 把某个 futex entry 的 `owner_dead` 置位，那么 waiter 被唤醒后会：

- `swap(false, Ordering::SeqCst)`
- 然后返回 `EOWNERDEAD`

也就是说，owner-dead 不是在退出路径直接返回给别人的，而是在后续 waiter 继续执行 `WAIT` 返回时观察到。

### 4.2 `WAKE` / `WAKE_BITSET`

唤醒路径会：

- 查 key 对应的 futex entry
- 若存在，则按 count 和 bitset mask 过滤 waiter
- 调用 `wake()`
- 之后 `ax_task::yield_now()`

`wake()` 的 bitset 语义是：

- waiter 记录自己的 bitset
- 唤醒方提供 mask
- `(bitset & mask) != 0` 的 waiter 才会被选中

所以 `WAIT_BITSET` / `WAKE_BITSET` 完全值得单独做一组位掩码测试，不只是“普通 wait/wake 的变种”。

### 4.3 `REQUEUE` / `CMP_REQUEUE`

这块实现已经存在，但当前仓库没有现成测试覆盖。

逻辑摘要：

- `FUTEX_CMP_REQUEUE` 先比较 `uaddr` 当前值是否等于 `value3`
- 若不等，返回 `WouldBlock`
- `timeout` 这个参数槽在 requeue 语义下被当成 `value2`
- 先从源队列 `wake(value)` 个 waiter
- 如果恰好唤醒数等于 `value`，再把最多 `value2` 个 waiter requeue 到目标队列

这块代码很容易因为 ABI 参数复用而读错，因此如果后面你开始补测试，我会优先把它列成“高收益但当前未覆盖”的用例。

## 5. `get_robust_list` / `set_robust_list` 当前实现

这两个 syscall 的表面 API 很简单，但它们真正的价值不在“登记动作”，而在退出路径能否正确消费这份登记。

### 5.1 `sys_set_robust_list`

当前行为：

- 要求 `size == size_of::<robust_list_head>()`
- 不相等就返回 `InvalidInput`
- 相等则把 head 地址写进 `current().as_thread().robust_list_head`

可以看到它现在并不：

- 遍历或校验链表内容
- 检查 head 指针是否可读
- 检查链表是否形成环

它本质上只是“登记一个线程私有 head 指针”。

### 5.2 `sys_get_robust_list`

当前行为：

- 通过 `get_task(tid)` 找目标线程
- `tid == 0` 时会返回当前线程
- 把目标线程保存的 `robust_list_head` 写回用户指针
- 同时把 `size_of::<robust_list_head>()` 写回 size 指针

这说明：

- 它不是进程级查询，而是线程级查询
- 目标线程必须还在 task table 里可见

### 5.3 目前“登记”之后真正发生了什么

答案在退出路径。

如果只看 `sys_get/set_robust_list`，这块工作量确实很小；但一旦把线程退出和 futex owner-dead 一起看，它就不再只是两个 getter/setter。

## 6. 线程退出路径上的真实语义

关键逻辑在：

- `os/StarryOS/kernel/src/task/ops.rs`

线程退出 `do_exit()` 时，当前顺序是：

1. 处理 `clear_child_tid`
2. 处理 `robust_list_head`
3. 再继续进程/线程退出、父子通知等收尾动作

这个顺序值得测试时关注。

### 6.1 `clear_child_tid` 路径

退出时会：

1. 取出 `thr.clear_child_tid()`
2. 尝试把该用户地址写成 `0`
3. 如果写成功，就按这个地址构造 futex key
4. 若该 futex entry 存在，则 `wake(1, u32::MAX)`
5. 然后 `yield_now()`

这条路径和 Linux 线程库语义直接相关：

- `set_tid_address`
- `CLONE_CHILD_CLEARTID`
- 线程 join / 回收等待

也就是说，虽然它不属于 `futex` syscall 本身，但在实际用户态同步里是强关联路径。

### 6.2 robust list 退出扫描

`exit_robust_list()` 的实现已经相当接近一个真正的机制，而不只是占位。

它会：

1. 从 `robust_list_head` 读取 head
2. 拿到：
   - `list.next`
   - `futex_offset`
   - `list_op_pending`
3. 遍历链表，直到回到 head
4. 对每个 entry 计算实际 futex word 地址
5. 找到对应 futex entry
6. 置 `owner_dead = true`
7. `wake(1, u32::MAX)`
8. 对 `list_op_pending` 单独补处理

这里和 changelog 里的这条记录直接对应：

- `process pending futex entry in exit_robust_list`

说明 `list_op_pending` 以前确实是个出过问题的分支，不是无关细节。

### 6.3 `handle_futex_death()` 的语义

这一步不是简单“广播唤醒”，而是：

- 根据 `entry + futex_offset` 算出 futex word 地址
- 找到对应 futex entry
- 置 `owner_dead = true`
- `wake(1, u32::MAX)`

因此当前实现体现的是：

- owner 死亡时，只唤醒 1 个 waiter
- waiter 通过后续 `WAIT` 返回观察到 `EOWNERDEAD`

这块非常适合做“有机制含量”的测试。

## 7. 与 `clone()` / `set_tid_address()` / `execve()` 的联动

## 7.1 `set_tid_address`

`sys_set_tid_address()` 的实现非常直接：

- 仅仅把地址存进 `Thread.clear_child_tid`
- 然后返回当前线程 ID

它不做额外校验，也不立即写内存。

真正的“清零并 futex 唤醒”发生在线程退出时。

## 7.2 `clone(... CLONE_CHILD_CLEARTID ...)`

`syscall/task/clone.rs` 中：

- 若带 `CLONE_CHILD_CLEARTID`
- 子线程创建时会把 `child_tid` 记到 `thr.set_clear_child_tid(child_tid)`

这条路径和 `set_tid_address` 本质上共享同一个退出时行为：

- child 退出时清零该地址
- 对该地址做一次 futex wake

因此如果后面你补测试，`set_tid_address` 和 `CLONE_CHILD_CLEARTID` 最好分开测，不要默认它们完全等价。

## 7.3 `execve()` 的现状

这里有一个值得记录的观察：

- `execve()` 现在会显式执行 `curr.as_thread().set_clear_child_tid(0)`
- 我没有在同一路径找到对 `robust_list_head` 的对应清零

仓库内搜索结果显示：

- `clear_child_tid` 在 `execve.rs` 里被重置
- `robust_list_head` 没有发现对应 reset 路径

这至少说明两件事：

1. 项目维护者已经意识到 `clear_child_tid` 里的旧地址在 `execve()` 后可能失效。
2. `robust_list_head` 当前是否也应在 `execve()` 后重置，值得专门确认。

我这里先把它记成“当前实现观察到的非对称点”，不直接下结论说它一定是 bug。后面正式测之前，建议先对照 Linux 行为确认一次。

## 8. 当前实现的几个重要特点

### 8.1 这是“有机制”的 futex，不是 stub

当前 StarryOS 在 futex 上已经不是简单占位：

- 有真实 wait queue
- 有 bitset wait/wake
- 有 requeue / cmp_requeue
- 有 robust list owner-dead
- 有 `clear_child_tid` 联动唤醒
- 有共享映射对象级 key 归并

这意味着补测试时是有机会测出机制类问题的。

### 8.2 当前测试覆盖明显不足

虽然机制已经有了，但测试现状很薄：

- 只有一个 `bug-futex-wait-wake` 回归用例
- 没有看到 dedicated `get_robust_list` / `set_robust_list` 测试
- 没有看到 owner-dead / robust list 退出路径测试
- 没有看到 `WAIT_BITSET` / `WAKE_BITSET` / `REQUEUE` / `CMP_REQUEUE` 测试

所以这块现在更像“代码实现已存在，但系统性验证还没跟上”。

### 8.3 `FUTEX_PRIVATE_FLAG` 目前没有显式参与 key 分流

用户态常见会传：

- `FUTEX_WAIT | FUTEX_PRIVATE_FLAG`
- `FUTEX_WAKE | FUTEX_PRIVATE_FLAG`

当前实现里：

- command 是通过 `futex_op & FUTEX_CMD_MASK` 提取的
- key 是否 private/shared 则是通过地址映射后端判断的

换句话说，当前实现不是“按 PRIVATE_FLAG 决定 private/shared”，而是“按地址所在映射类型决定 private/shared”。

这个设计不一定错，但它是一个很值得明确知道的项目特征。

## 9. 当前值得优先怀疑和测试的点

下面这些点不是已经确认的 bug，而是我在调研时认为最值得优先验证的地方。

### 9.1 timeout 边界

重点看：

- `FUTEX_WAIT` 的相对超时
- `FUTEX_WAIT_BITSET` 的绝对超时
- `FUTEX_CLOCK_REALTIME` 分支

这块很容易因为时间源、超时负值、绝对/相对搞混而出错。

### 9.2 bitset 语义

重点看：

- waiter bitset 和 wake mask 的交集过滤是否正确
- `value3 == 0` 之类的非法 bitset 场景是否做了 Linux 对齐校验

我在当前实现里没有看到针对 bitset 为 0 的显式检查，这值得测。

### 9.3 requeue 语义

重点看：

- `FUTEX_REQUEUE`
- `FUTEX_CMP_REQUEUE`
- 比较值不匹配时返回路径
- requeue 数量边界

这是当前最像“已经实现但没有系统验证”的一块。

### 9.4 robust list owner-dead 路径

重点看：

- 链表扫描是否能正确回到 head
- `list_op_pending` 是否正确补处理
- waiter 是否确实观察到 `EOWNERDEAD`
- 只唤醒 1 个 waiter 的语义是否符合预期

### 9.5 `execve()` 后的 `robust_list_head`

这是当前最值得单独确认的设计点之一：

- `clear_child_tid` 已清零
- `robust_list_head` 没看到对应 reset

如果 Linux 在 `execve()` 后会清掉 robust list，而 StarryOS 不清，那么这里可能会是后续真实问题来源。

## 10. 现有测试与变更记录

### 10.1 现有 StarryOS 用例

- `test-suit/starryos/normal/qemu-smp1/bugfix/bug-futex-wait-wake`
  - 覆盖曾经的 futex 等待回归
  - 场景很窄
  - 只证明基本的 waiter 能阻塞并被 wake

### 10.2 changelog 里的直接相关项

`os/StarryOS/kernel/CHANGELOG.md` 中目前和这组最相关的是：

- `fix futex wait user-memory access`
- `process pending futex entry in exit_robust_list`

如果后面你要继续修这块，建议先把对应 PR 也找出来看提交上下文。

## 11. 建议的阅读顺序

如果你的目标是“在正式写测试前先熟悉这块实现”，我建议按下面顺序读：

1. `os/StarryOS/kernel/src/syscall/sync/futex.rs`
   - 先看 syscall 面上的支持范围与返回路径。

2. `os/StarryOS/kernel/src/task/futex.rs`
   - 再看 wait queue、key、private/shared、requeue 的内部对象模型。

3. `os/StarryOS/kernel/src/task/ops.rs`
   - 再看线程退出时怎么联动 `clear_child_tid` 与 robust list。

4. `os/StarryOS/kernel/src/syscall/task/clone.rs`
   - 看 `CLONE_CHILD_CLEARTID` 是怎么接入线程对象的。

5. `os/StarryOS/kernel/src/syscall/task/execve.rs`
   - 重点关注为什么 `clear_child_tid` 会被清零，以及 `robust_list_head` 当前没有对应动作。

6. `docs/docs/crates/starry-vm.md`
   - 如果你准备修用户地址访问相关 bug，这份要一起读。

## 12. 对后续测试设计的直接启发

结合当前实现，我认为后续测试不要只按 syscall 名字拆，而应该按机制拆：

- 基础等待/唤醒
  - `WAIT` / `WAKE`
  - 值不匹配
  - timeout

- bitset 语义
  - `WAIT_BITSET` / `WAKE_BITSET`

- requeue 语义
  - `REQUEUE` / `CMP_REQUEUE`

- robust list 登记与读取
  - `set_robust_list`
  - `get_robust_list`
  - 非法 size / 非法 tid / 坏指针

- 退出路径同步语义
  - `clear_child_tid`
  - robust owner-dead
  - `EOWNERDEAD`

也就是说，真正有价值的工作量几乎都不在 `get/set_robust_list` 本身，而在它们和 futex、线程退出路径之间的联动。

## 13. 本次调研的结论

一句话总结：

- StarryOS 这组 syscall 目前已经有“可工作的机制骨架”，不是 stub。
- 代码复杂度主要集中在 `futex` 机制、共享 key、退出路径、owner-dead，而不是 `get/set_robust_list` 本身。
- 当前最明显的短板不是“没实现”，而是“测试覆盖不够系统”。
- 在正式写测试和修复前，最值得优先确认的开放点是：
  - timeout 语义
  - bitset 边界
  - requeue 行为
  - robust owner-dead 路径
  - `execve()` 后 `robust_list_head` 的预期语义

