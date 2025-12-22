/// # Tokio ↔ std::io 对应关系示例
///
/// | Tokio 方法                    | std::io 方法          | 说明                     |
/// |------------------------------|----------------------|-------------------------|
/// | `tokio::io::copy`            | `std::io::copy`      | 异步复制 Reader → Writer |
/// | `AsyncReadExt::read`         | `Read::read`         | 异步读取数据             |
/// | `AsyncReadExt::read_exact`   | `Read::read_exact`   | 异步读取固定长度数据     |
/// | `AsyncWriteExt::write`       | `Write::write`       | 异步写入数据             |
/// | `AsyncWriteExt::write_all`   | `Write::write_all`   | 异步写入完整缓冲区       |
/// | `AsyncWriteExt::flush`       | `Write::flush`       | 异步刷新缓冲区           |
// AsyncRead and AsyncWrite
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// File 实现了 AsyncRead trait
// 但是他也能调用 AsyncReadExt(继承了AsyncRead的子的trait)
#[tokio::test]
async fn async_fn_read() -> io::Result<()> {
    let mut f = File::open("../../../Cargo.toml").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    // &buffer[..n] 这是一个 切片操作：
    // buffer 是一个数组（比如 [0; 10]）。
    // [..n] 表示从索引 0 到 n（不包括 n）的部分。
    // &buffer[..n] 是对这个切片的引用。
    // 如果只有 [..] 而没有 n，那表示整个数组或整个切片的范围
    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

#[tokio::test]
async fn async_fn_read_to_end() -> io::Result<()> {
    let mut f = File::open("../../../Cargo.toml").await?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;
    Ok(())
}

#[tokio::test]
async fn async_fn_write() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);
    Ok(())
}
#[tokio::test]
async fn async_fn_write_all() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    file.write_all(b"some bytes").await?;
    Ok(())
}

// Additionally, just like std, the tokio::io module contains a number of helpful utility functions as well as APIs for working with standard input,
// standard output and standard error.
// For example, tokio::io::copy asynchronously copies the entire contents of a reader into a writer.
#[tokio::test]
async fn io_copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}

#[tokio::test]
async fn echo_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // vec![x; n] 用分号表示 重复 n 次相同的元素。
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` signifies that the remote has
                    // closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected socket error. There isn't much we can
                            // do here so just stop processing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. There isn't much we can do
                        // here so just stop processing.
                        return;
                    }
                }
            }
        });
    }
}

#[tokio::test]
async fn echo_client() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);
    // 又要从socket读 又要写到socket去 这里是不好实现
    // io::copy(&mut socket, &mut socket).await
    // 需要把socket split成 一个reader 一个writer
    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        // Sometimes, the rust type inferencer needs
        // a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}
