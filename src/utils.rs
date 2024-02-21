use std::mem;

use tokio::select;
use tokio::signal::ctrl_c;

pub(super) async fn shutdown_signal() {
  let ctrl_c = async { ctrl_c().await.expect("failed to install Ctrl+C handler") };

  #[cfg(unix)]
  {
    let terminate = async {
      tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    };

    select! {
      _ = ctrl_c => {},
      _ = terminate => {},
    }
  }

  #[cfg(not(unix))]
  {
    ctrl_c.await;
    Ok(())
  }
}

const MAX_DATAGRAM_SIZE: usize = u16::MAX as usize - mem::size_of::<u16>();

/// Creates and returns a buffer on the heap with enough space to contain any possible
/// UDP datagram.
///
/// This is put on the heap and in a separate function to avoid the 64k buffer from ending
/// up on the stack and blowing up the size of the future using it.
#[inline(never)]
pub(super) fn datagram_buffer() -> Box<[u8; MAX_DATAGRAM_SIZE]> {
  Box::new([0u8; MAX_DATAGRAM_SIZE])
}
