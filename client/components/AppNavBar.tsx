import { Navbar, Button, Classes } from '@blueprintjs/core';
import Link from 'next/link';

export const AppNavBar = () => {
  return (
    <Navbar fixedToTop>
      <Navbar.Group>
        <Navbar.Heading>
          <Link href="/">
            Waki
          </Link>
        </Navbar.Heading>
        <Navbar.Divider />
        <Button className={Classes.MINIMAL} text="Home" />
        <Button className={Classes.MINIMAL} text="Pages" />
      </Navbar.Group>
    </Navbar>
  );
};
