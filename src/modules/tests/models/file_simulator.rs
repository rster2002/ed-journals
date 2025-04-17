use futures::AsyncWrite;

pub struct FileSimulator<T>
where T : AsyncWrite
{
    inner: T,
    content: String,
}

impl<T> FileSimulator<T>
where T : AsyncWrite
{
    pub fn new(inner: T, content: String) -> FileSimulator<T> {
        FileSimulator {
            inner,
            content,
        }
    }

    pub fn step(&self) {

    }
}
