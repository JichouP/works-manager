import { VFC } from 'react';
import * as React from 'react';
import Navbar from './Navbar';
import Sidebar from './Sidebar';

type Props = {
  children: React.ReactNode;
};

const Layout: VFC<Props> = ({ children }) => {
  return (
    <>
      <div className='w-full h-screen rounded drawer drawer-mobile'>
        <input id='drawer-sidebar' type='checkbox' className='drawer-toggle' />
        <div className='drawer-content'>
          <Navbar title={'Works Manager'}></Navbar>
          <div className='p-4'>{children}</div>
        </div>
        <Sidebar></Sidebar>
      </div>
    </>
  );
};

export default Layout;
