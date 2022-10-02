import "../styles/globals.css";
import type { AppProps } from "next/app";

function Waki({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}

export default Waki;
