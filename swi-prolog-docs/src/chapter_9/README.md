# 复用顶层绑定

顶层目标成功执行后得到的绑定，如果它们不太大，会被断言到一个数据库中。这里的“不太大”由 Prolog 标志 `toplevel_var_size` 定义。这些值可以在后续顶层查询中以 `$Var` 的形式复用。如果后续查询使用了同名变量，系统会把该变量关联到最近一次绑定。示例：

```prolog
1 ?- maplist(plus(1), `hello`, X).
X = [105,102,109,109,112].

2 ?- format('~s~n', [$X]).
ifmmp
true.

3 ?-
```

注意，也可以通过执行 `=/2` 来设置变量：

```prolog
6 ?- X = statistics.
X = statistics.

7 ?- $X.
% Started at Fri Aug 24 16:42:53 2018
% 0.118 seconds cpu time for 456,902 inferences
% 7,574 atoms, 4,058 functors, 2,912 predicates, 56 modules, 109,791 VM-codes
%
%                     Limit   Allocated      In use
% Local  stack:           -       20 Kb    1,888  b
% Global stack:           -       60 Kb       36 Kb
% Trail  stack:           -       30 Kb    4,112  b
%        Total:    1,024 Mb      110 Kb       42 Kb
%
% 3 garbage collections gained 178,400 bytes in 0.000 seconds.
% 2 clause garbage collections gained 134 clauses in 0.000 seconds.
% Stack shifts: 2 local, 2 global, 2 trail in 0.000 seconds
% 2 threads, 0 finished threads used 0.000 seconds
true.
```
