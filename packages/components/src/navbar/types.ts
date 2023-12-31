import type { VNodeChild } from 'vue';
import type { RouteLocationRaw } from 'vue-router';
import type { IconLinkProps } from '../link';

export interface NavbarLogoProps {
  logo: string | (() => VNodeChild);
}

export interface NavbarTitleProps {
  title: string | (() => VNodeChild);
}

export interface NavbarMenuItem {
  to?: RouteLocationRaw;
}

export interface NavbarMenuProps {
  menu?: NavbarMenuItem[];
}

export type NavbarChildrenProps = Partial<NavbarLogoProps> &
  Partial<NavbarTitleProps> &
  Partial<NavbarMenuProps>;

export interface NavbarProps extends NavbarChildrenProps {
  socialLinks?: IconLinkProps[];
  titleLink?: RouteLocationRaw;
}