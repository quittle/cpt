let evtSource: EventSource;

export function init(onConnected?: () => void) {
  evtSource = new EventSource("/sse");
  evtSource.onopen = onConnected ?? null;

  evtSource.onerror = (err) => {
    console.error("EventSource failure:", err);
  };
}

export function addEventListener(
  event: string,
  callback: (MessageEvent) => void,
) {
  evtSource.addEventListener(event, callback);
}

export function removeEventListener(
  event: string,
  callback: (MessageEvent) => void,
) {
  evtSource.removeEventListener(event, callback);
}
