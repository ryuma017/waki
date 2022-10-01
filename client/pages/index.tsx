import type { NextPage } from "next";
import { useGetPageByTitleQuery } from "../generated/graphql";

const Home: NextPage = () => {
  const { data } = useGetPageByTitleQuery({
    variables: {
      title: "Hi👋🏻, I'm ryuma017",
    },
  });

  return (
    <>
      <h1>{data?.pageByTitle?.title}</h1>
      <div dangerouslySetInnerHTML={{ __html: data?.pageByTitle?.bodyHtml ?? "" }}></div>
    </>
  );
};

export default Home;
