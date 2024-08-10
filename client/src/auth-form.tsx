import { Component, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { A, Navigator, useNavigate } from "@solidjs/router";

const [email, setEmail] = createSignal<string>();
const [password, setPassword] = createSignal<string>();

const register = async (event: SubmitEvent) => {
  event.preventDefault();

  const [salt, verifier] = await invoke<[number[], number[]]>(
    "create_salt_and_verifier",
    {
      username: email(),
      password: password(),
    },
  );

  await fetch(`http://localhost:3000/register`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email: email(),
      salt,
      verifier,
    }),
  });
};

const login = async (event: SubmitEvent, navigate: Navigator) => {
  event.preventDefault();

  try {
    const ws = new WebSocket("ws://localhost:3000/login");
    const public_a = await invoke<number[]>("public_a");

    if (ws.readyState === 1) {
      ws.send(JSON.stringify({ email: email() }));
      ws.send(JSON.stringify(public_a));
    }

    ws.addEventListener("message", async (event) => {
      const data = JSON.parse(event.data);

      switch (data.type) {
        case "first_step_server":
          const clientProof = await invoke<number[]>("compute_verifier", {
            username: email(),
            password: password(),
            salt: data.salt,
            publicB: data.public_b,
          });
          ws.send(JSON.stringify(clientProof));
          break;
        case "second_step_server":
          await invoke("verify_server_proof", {
            serverProof: data.server_proof,
          });

          navigate("/passwords");

          break;
      }
    });
  } catch (error) {
    console.error(error);
  }
};

type AuthFormProps = {
  onSubmit: (event: SubmitEvent) => void;
};

const AuthForm: Component<AuthFormProps> = (props) => (
  <form
    class="flex flex-col gap-2 p-10 outline-white outline"
    onSubmit={props.onSubmit}
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

export const LoginPage: Component = () => {
  const navigate = useNavigate();

  return (
    <>
      <h3 class="text-white mb-2">Login</h3>
      <AuthForm onSubmit={(e) => login(e, navigate)} />
      <A class="mt-2 text-white" href="/register">
        Register here.
      </A>
    </>
  );
};

export const RegisterPage: Component = () => (
  <>
    <h3 class="text-white mb-2">Register</h3>
    <AuthForm onSubmit={register} />
    <A class="mt-2 text-white" href="/">
      Login here.
    </A>
  </>
);
