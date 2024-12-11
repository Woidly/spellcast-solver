type GMRequestDetails = {
  url: string;
  method: string;
};

type GMRequestResponse = {
  responseText: string;
};

declare global {
  var GM: {
    xmlHttpRequest(details: GMRequestDetails): Promise<GMRequestResponse>;
  };
}

export function httpRequest(url: string, method: string): [Promise<string>, () => void] {
  let resolve: (value: string) => void;
  let reject: (error: Error) => void;
  let promise: Promise<string> = new Promise((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });
  let interrupt = () => {
    reject(new Error("Interrupted"));
  };
  if (typeof GM !== "undefined" && typeof GM.xmlHttpRequest !== "undefined") {
    GM.xmlHttpRequest({ url, method }).then((x) => resolve(x.responseText));
  } else {
    fetch(url, { method })
      .then((x) => x.text())
      .then((x) => resolve(x));
  }
  return [promise, interrupt];
}
