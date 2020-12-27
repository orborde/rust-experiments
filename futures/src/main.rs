use futures::{
    future::FutureExt, // for `.fuse()`
    future::TryFutureExt,
    executor,
    pin_mut,
    join,
};

async fn task_one() -> Result<&'static str,&'static str> { Ok("ONE") }
async fn task_two() -> Result<&'static str,&'static str> { Ok("TWO") }

async fn race_tasks() {
    let t1 = task_one();
    let t2 = task_two();

    //pin_mut!(t1, t2);

    join!(
        t1.and_then(|_| async { println!("task one completed first"); Ok("done") }),
        t2.and_then(|_| async { println!("task two completed first"); Ok("done") }),
    );
}

fn main() {
    executor::block_on(race_tasks());
}
