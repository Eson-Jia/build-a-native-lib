// 注意这里没有 `#[link]`属性了,我们将选择链接哪一个库的任务委派给了编译脚本,而不再是将其硬编码到源码中.
extern { fn hello(); }

fn main() {
    unsafe { hello(); }
}
