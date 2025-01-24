import { createSignal } from "solid-js";
import axios from "axios";
import { authCookie, setAuthCookie, AuthCookieData } from "../../utils/Cookies";

function Login() {
  const [email, setEmail] = createSignal<string>("");
  const [password, setPassword] = createSignal<string>("");
  const [error, setError] = createSignal<string>("");
  const [loading, setLoading] = createSignal<boolean>(false);

  const backendUrl =
    import.meta.env.VITE_BACKEND_URL || "http://localhost:3000";
  console.log(`Backend URL: ${backendUrl}/auth/login`);

  async function handleLogin(event: { preventDefault: () => void }) {
    event.preventDefault(); // Prevent the default form submission

    setLoading(true);
    setError("");

    try {
      const response = await axios.post(`${backendUrl}/auth/login`, {
        email: email(),
        password: password(),
      });

      console.log("Login successful:", response.data);
      const { token, user_email, user_id, user_username } = response.data;
      setAuthCookie({
        token,
        user_email,
        user_id,
        user_username,
      } as AuthCookieData);
    } catch (error: unknown) {
      console.error("Login failed:", error);

      if (
        axios.isAxiosError(error) &&
        error.response &&
        error.response.status === 401
      ) {
        setError("Invalid email or password.");
      } else {
        setError("An unexpected error occurred. Please try again later.");
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <div class="h-screen w-screen flex justify-center items-center bg-gray-100">
      <form
        onSubmit={handleLogin}
        class="bg-white p-6 rounded-lg shadow-md flex flex-col gap-y-4"
      >
        <h2 class="text-2xl font-bold mb-4">Login</h2>

        {error() && <div class="text-red-500">{error()}</div>}

        <div class="flex flex-col">
          <label for="email" class="mb-1 text-gray-600">
            Email:
          </label>
          <input
            class="border border-gray-300 rounded-lg px-3 py-2 text-lg"
            type="email"
            id="email"
            name="email"
            placeholder="john.doe@example.com"
            value={email()}
            onInput={(e) => setEmail(e.currentTarget.value)}
            required
          />
        </div>

        <div class="flex flex-col">
          <label for="password" class="mb-1 text-gray-600">
            Password:
          </label>
          <input
            class="border border-gray-300 rounded-lg px-3 py-2 text-lg"
            type="password"
            id="password"
            name="password"
            placeholder="Your secure password"
            value={password()}
            onInput={(e) => setPassword(e.currentTarget.value)}
            required
          />
        </div>

        <button
          type="submit"
          class={`w-full bg-black text-white py-2 rounded-lg hover:bg-black/85 transition-colors ${
            loading() ? "opacity-50 cursor-not-allowed" : ""
          }`}
          disabled={loading()}
        >
          {loading() ? "Logging in..." : "Login"}
        </button>
      </form>
    </div>
  );
}

export default Login;
