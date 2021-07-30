import React, { useState } from "react";
import { Redirect } from "react-router-dom";
import { CopyToClipboard } from "react-copy-to-clipboard";

export default function Home() {
  const [cp, setCp] = useState(false);
  let pubkey = window.localStorage.getItem("pubkey");
  if (pubkey === null || pubkey === undefined || pubkey === "") {
    return <Redirect to="/signup" />;
  } else {
    return (
      <div className="px-4 py-8 grid grid-cols-1 gap-8 items-stretch">
        <div className="text-center">
          <div className="text-2xl font-black p-4">SEL</div>
          <div className="font-bold">Magic Protocol for Everyone</div>
        </div>
        <div className="">
          <div className="font-bold">PUBLIC KEY</div>
        </div>
        <div className="border-4 border-yellow-900 w-full h-auto block bg-transparent p-4 break-all rounded-2xl ">
          {pubkey}
        </div>
        {/* <div
          className="border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black"
          onClick={(e) => document.execCommand()}
        >
          COPY
        </div> */}
        <CopyToClipboard text={pubkey} onCopy={() => setCp(true)}>
          <div className="border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black cursor-pointer">
            {cp ? "COPIED!" : "COPY"}
          </div>
        </CopyToClipboard>
        <div
          className="border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black cursor-pointer"
          onClick={(e) => {
            window.localStorage.removeItem("pubkey");
            window.location.replace("/");
          }}
        >
          SIGN OUT
        </div>
      </div>
    );
  }
}
