use celery::task::TaskResult;

#[celery::task]
pub(crate) fn add(x: i32, y: i32) -> TaskResult<i32> {
    let res = x + y;
    println!("{:?}", res);
    Ok(res)
}

#[celery::task]
pub(crate) fn long_running_task(secs: Option<u64>) -> TaskResult<()> {
    println!("{:?}", secs);
    unimplemented!()
}