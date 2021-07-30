import Logo from "./assets/logo.svg";

export default function SignUp() {
  return (
    <div className="SignUp">
      <div>
        <div className="text-3xl font-black p-4">SEL</div>
        <div className="text-lg font-bold">Magic Protocol for Everyone</div>
      </div>
      <div
        className="bg-center bg-contain bg-no-repeat"
        style={{ height: "50vh", backgroundImage: `url("${Logo}")` }}
      ></div>
      <div>
        <button
          className="py-4 px-8 bg-yellow-700 rounded-2xl border-4 border-yellow-900 font-bold text-xl"
          onClick={(e) => {
            window.webkit.messageHandlers.sel.postMessage("genkey");
            setTimeout(() => {
              window.location.replace("/");
            }, 2000);
          }}
        >
          START
        </button>
      </div>
    </div>
  );
}
