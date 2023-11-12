import { Component } from "solid-js";
import { createStore } from "solid-js/store";

type RegisterFormFields = {
  firstName: string;
  lastName: string;
  displayName: string;
  email: string;
  password: string;
};

export const RegisterForm: Component = () => {
  const [fields, setFields] = createStore<RegisterFormFields>({
    firstName: "",
    lastName: "",
    displayName: "",
    email: "",
    password: "",
  });

  const onSubmit = async (event: SubmitEvent) => {
    event.preventDefault();

    try {
      if (!fields.email || !fields.password) {
        throw new Error("Invalid Fields");
      }

      const response = await fetch("http://localhost:3000/register", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(fields),
      });

      if (response.ok) {
        console.log("OK");
      }
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <form class="flex flex-col gap-2 p-4" onSubmit={onSubmit}>
      <input
        name="firstName"
        placeholder="First name"
        onInput={(e) => setFields("firstName", e.currentTarget.value)}
      />
      <input
        name="lastName"
        placeholder="Last name"
        onInput={(e) => setFields("lastName", e.currentTarget.value)}
      />
      <input
        name="displayName"
        placeholder="Display Name"
        onInput={(e) => setFields("displayName", e.currentTarget.value)}
      />
      <input
        name="email"
        type="email"
        placeholder="Email"
        onInput={(e) => setFields("email", e.currentTarget.value)}
      />
      <input
        name="password"
        type="password"
        placeholder="Password"
        onInput={(e) => setFields("password", e.currentTarget.value)}
      />
      <button type="submit">Submit</button>
    </form>
  );
};
