import { H1 } from "@blueprintjs/core";
import type { GetServerSidePropsContext, NextPage } from "next";
import { AppNavBar } from "../../components/AppNavBar";
import { Page } from "../../generated/graphql";
import { sdk } from "../../src/client";

export async function getServerSideProps(context: GetServerSidePropsContext) {
  const { title } = context.query;
  const titleString = (title as string[]).join("/");
  const data = await sdk.getPageByTitle({ title: titleString });
  const page = data.pageByTitle;
  return {
    props: {
      page,
    },
  };
}

const Page: NextPage<{ page: Page }> = ({ page }) => {
  return (
    <>
      <AppNavBar />

      <div className="mx-auto max-w-screen-md">
        <H1>{page.title}</H1>
        <article className="markdown-body">
          <div dangerouslySetInnerHTML={{ __html: page.bodyHtml ?? "" }} />
        </article>
      </div>
    </>
  );
};

export default Page;
