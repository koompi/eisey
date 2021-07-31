import React, { useState } from "react";
import { Redirect } from "react-router-dom";
import { useMutation } from "react-query";
import { CopyToClipboard } from "react-copy-to-clipboard";
import axios from "axios";

export default function Home() {
  let token = window.localStorage.getItem("token");
  const [cp, setCp] = useState(false);
  const [signData, setSignData] = useState({
    token: "",
    user_pub_key: "",
    url: "",
  });
  const [signMessage, setSignMessage] = useState("");

  const mutation = useMutation(async (data) => {
    return await axios.post(
      "http://localhost:8080/api/sign",
      {
        ...signData,
        token: token,
      },
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    );
  });

  if (token === null || token === undefined || token === "") {
    return <Redirect to="/signin" />;
  } else {
    return (
      <div className="h-screen px-4 py-8 grid grid-cols-1 gap-8 items-stretch overflow-y-scroll">
        <div className="">
          <div className="font-bold">USER PUBLIC KEY</div>
        </div>
        <textarea
          className="border-4 border-yellow-900 w-full h-auto block bg-transparent p-4 break-all rounded-2xl text-white"
          value={signData.user_pub_key}
          onChange={(e) =>
            setSignData({
              ...signData,
              user_pub_key: e.target.value,
            })
          }
        />
        <div className="">
          <div className="font-bold">COMMAND</div>
        </div>
        <textarea
          className="border-4 border-yellow-900 w-full h-auto block bg-transparent p-4 break-all rounded-2xl "
          value={signData.url}
          onChange={(e) =>
            setSignData({
              ...signData,
              url: e.target.value,
            })
          }
        />
        <button
          disabled={signData.user_pub_key === "" || signData.url === ""}
          className={
            signData.user_pub_key === "" || signData.url === ""
              ? "cursor-not-allowed border-4 border-yellow-900 w-full h-auto block bg-yellow-900  text-gray-900 p-4 break-all rounded-2xl text-center font-black cursor-pointer"
              : "border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black cursor-pointer"
          }
          onClick={() => {
            mutation.mutate();
          }}
        >
          SIGN
        </button>
        {/* {signMessage && (
          <>
            <div className="border-4 border-yellow-900 w-full h-auto block bg-transparent p-4 break-all rounded-2xl ">
              {signMessage}
            </div>
            <CopyToClipboard text={token} onCopy={() => setCp(true)}>
              <div className="border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black cursor-pointer">
                {cp ? "COPIED!" : "COPY"}
              </div>
            </CopyToClipboard>
          </>
        )} */}
        {mutation.isLoading ? (
          <label className="text-yellow-700 font-medium p-3 bg-yellow-100 rounded-lg mb-4 break-words block">
            Logging in...
          </label>
        ) : (
          <>
            {mutation.isError ? (
              <div className="text-red-700 font-medium p-3 bg-red-100 rounded-lg mb-4 break-words block">
                {mutation.error.message}
              </div>
            ) : null}

            {mutation.isSuccess
              ? mutation.data.data && (
                  <>
                    <div className="border-4 border-yellow-900 w-full h-auto block bg-transparent p-4 break-all rounded-2xl">
                      {mutation.data.data}
                    </div>
                    <CopyToClipboard
                      text={mutation.data.data}
                      onCopy={() => setCp(true)}
                    >
                      <div className="border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black cursor-pointer">
                        {cp ? "COPIED!" : "COPY"}
                      </div>
                    </CopyToClipboard>
                  </>
                )
              : null}
          </>
        )}
        <div
          className="border-4 border-yellow-900 w-full h-auto block bg-yellow-700 p-4 break-all rounded-2xl text-center font-black cursor-pointer"
          onClick={(e) => {
            window.localStorage.removeItem("token");
            window.location.replace("/");
          }}
        >
          SIGN OUT
        </div>
        <div className="pb-4"></div>
      </div>
    );
  }
}
