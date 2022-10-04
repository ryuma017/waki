import type { GetServerSidePropsContext, NextPage } from "next";
import { AppNavBar } from "../components/AppNavBar";
import { Page } from "../generated/graphql";
import { sdk } from "../src/client";

export async function getServerSideProps() {
  const data = await sdk.getPageByTitle({ title: "Hi👋🏻, I'm ryuma017" });
  const page = data.pageByTitle;
  return {
    props: {
      page,
    },
  };
}

const Home: NextPage<{ page: Page }> = ({ page }) => {
  return (
    <>
      <AppNavBar />
      <h1>{page.title}</h1>
      <div dangerouslySetInnerHTML={{ __html: page.bodyHtml ?? "" }}></div>
    </>
  );
};

export default Home;
