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

export async function httpRequest(url: string, method: string): Promise<string> {
  if (typeof GM !== "undefined" && typeof GM.xmlHttpRequest !== "undefined") {
    return GM.xmlHttpRequest({ url, method }).then((x) => x.responseText);
  } else {
    return fetch(url, { method }).then((x) => x.text());
  }
}
