import { useState, useEffect } from "react";
import { useMutation } from "react-query";
import axios from "axios";
import Logo from "./assets/logo.svg";

export default function SignUp() {
  const [formData, setFormData] = useState({
    email: "",
    password: "",
  });
  const mutation = useMutation(async (data) => {
    return await axios.post("http://localhost:8080/sign_in", data, {
      headers: {
        "Content-Type": "application/json",
        credential: "same-origin",
      },
    });
  });
  const redirect = (token) => {
    window.localStorage.setItem("token", token);
    setTimeout(() => window.location.replace("/"), 1500);
  };
  useEffect(() => {
    console.log(formData);
  });
  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        mutation.mutate(formData);
      }}
    >
      <div className="SignUp">
        <div>
          <div className="text-3xl font-black p-4">SEL Admin</div>
          <div className="text-lg font-bold">Magic Protocol for Everyone</div>
        </div>
        <div
          className="bg-center bg-contain bg-no-repeat"
          style={{ height: "15vh", backgroundImage: `url("${Logo}")` }}
        ></div>
        <div className="border-4 border-yellow-700  00 rounded-xl p-2">
          <label htmlFor="email" className="block text-left text-xs">
            Email
          </label>
          <input
            type="email"
            name="email"
            className="block w-full bg-transparent"
            value={formData.email}
            onChange={(e) =>
              setFormData({ ...formData, email: e.target.value })
            }
          />
        </div>
        <div className="border-4 border-yellow-700 rounded-xl p-2">
          <label htmlFor="password" className="block text-left text-xs">
            Password
          </label>
          <input
            type="password"
            name="password"
            className="block w-full bg-transparent"
            value={formData.password}
            onChange={(e) =>
              setFormData({ ...formData, password: e.target.value })
            }
          />
        </div>
        <div className="border-4 border-yellow-700 rounded-xl overflow-hidden">
          <input
            type="submit"
            value="SIGN IN"
            className="p-4 block w-full bg-yellow-900 text-lg font-bold"
          />
        </div>
        {mutation.isLoading ? (
          <label className="text-yellow-700 font-medium p-3 bg-yellow-100 rounded-lg mb-4 break-words block">
            Logging in...
          </label>
        ) : (
          <>
            {mutation.isError ? (
              <label
                htmlFor="submit"
                className="text-red-700 font-medium p-3 bg-red-100 rounded-lg mb-4 break-words block"
              >
                {mutation.error}
              </label>
            ) : null}

            {mutation.isSuccess ? (
              mutation.data.data.token ? (
                <label
                  htmlFor="submit"
                  className="text-green-700 font-medium p-3 bg-green-100 rounded-lg mb-4 break-words block"
                >
                  Login successfully.
                </label>
              ) : (
                <>
                  {mutation.data.data && (
                    <label
                      htmlFor="submit"
                      className="text-red-700 font-medium p-3 bg-red-100 rounded-lg mb-4 break-words block"
                    >
                      {mutation.data.data}
                    </label>
                  )}
                </>
              )
            ) : null}
          </>
        )}
      </div>
      {mutation.isSuccess && mutation.data.data.token
        ? redirect(mutation.data.data.token)
        : null}
    </form>
  );
}
