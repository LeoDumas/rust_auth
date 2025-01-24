import { createServerCookie } from "@solid-primitives/cookies";

export interface AuthCookieData {
  token?: string;
  user_email?: string;
  user_id?: number;
  user_username?: string;
}

export const [authCookie, setAuthCookie] = createServerCookie<AuthCookieData>("authCookie", {
  // How to serialize our JavaScript object to a string (saved in the browser)
  serialize: (val) => {
    // Provide a fallback for empty or null values
    if (!val || typeof val !== "object") return "";
    return JSON.stringify(val);
  },

  // How to read that string back into a JavaScript object
  deserialize: (str?: string) => {
    try {
      return JSON.parse(str || "{}") as AuthCookieData;
    } catch (err) {
      return {};
    }
  },

  // Maximum age for the cookie in seconds : 1d
  cookieMaxAge: 60 * 60 * 24,
});
