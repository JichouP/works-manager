import { createContext, useState, VFC } from 'react';
import Layout from './components/Layout';
import Home from './pages/home';

type Pages = 'home';

const RouterContext = createContext<[Pages, (page: Pages) => void]>([
  'home',
  () => {},
]);

type SelectorProps = {
  page: Pages;
};

const Selector: VFC<SelectorProps> = ({ page }) => {
  switch (page) {
    case 'home':
      return <Home></Home>;

    default:
      return <></>;
  }
};

const Router: VFC = () => {
  const [page, setPage] = useState<Pages>('home');

  return (
    <RouterContext.Provider value={[page, setPage]}>
      <Layout>
        <Selector page={page}></Selector>
      </Layout>
    </RouterContext.Provider>
  );
};

export default Router;
