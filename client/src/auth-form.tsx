import { Component, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

type AuthFormProps = {
  action: "login" | "register";
};

export const AuthForm: Component<AuthFormProps> = () => {
  const [email, setEmail] = createSignal<string>();
  const [password, setPassword] = createSignal<string>();

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    const res = await invoke("handle_submit", {
      username: email(),
      password: password(),
    });

    console.log(res);
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
        onInput={(e) => setEmail(e.currentTarget.value)}
      />
      <input
        class="rounded-md p-2"
        name="password"
        type="password"
        placeholder="Password"
        onInput={(e) => setPassword(e.currentTarget.value)}
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
