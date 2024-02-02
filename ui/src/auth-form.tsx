import { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { verifier } from "./utils";

type AuthFormFields = {
  email: string;
  password: string;
};

type AuthFormProps = {
  action: "login" | "register";
};

export const AuthForm: Component<AuthFormProps> = ({ action }) => {
  const [fields, setFields] = createStore<AuthFormFields>({
    email: "",
    password: "",
  });

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    try {
      if (!fields.email || !fields.password) {
        throw new Error("Invalid Fields");
      }

      const response = await fetch(`http://localhost:3000/${action}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(await encryptData(JSON.stringify(fields))),
      });
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <form
      class="flex flex-col gap-2 p-10 outline-white outline"
      onSubmit={onSubmit}
    >
      <input
        class="rounded-md p-2"
        name="email"
        type="email"
        placeholder="Email"
        onInput={(e) => setFields("email", e.currentTarget.value)}
      />
      <input
        class="rounded-md p-2"
        name="password"
        type="password"
        placeholder="Password"
        onInput={(e) => setFields("password", e.currentTarget.value)}
      />
      <button
        class="border outline-white rounded-md text-white bg-sky-900 p-2"
        type="submit"
      >
        Submit
      </button>
    </form>
  );
};
