use futures::{
    //future::FutureExt, // for `.fuse()`
    future::TryFutureExt,
    executor,
    pin_mut,
    join,
};

async fn task_one() -> Result<&'static str,&'static str> { println!("exec-one"); Ok("ONE") }
async fn task_two() -> Result<&'static str,&'static str> { println!("exec-two"); Ok("TWO") }

async fn race_tasks() {
    let t1 = task_one();
    let t2 = task_two();

    //pin_mut!(t1, t2);

    join!(
        //t1.map_ok(|_| async { println!("task one completed first") }),
        //t2.map_ok(|_| async { println!("task two completed first") }),
        async { t1.await.map(|_| println!("task one finished")) },
        async { t2.await.map(|_| println!("task two finished")) },
    );
}

fn main() {
    executor::block_on(race_tasks());
}
