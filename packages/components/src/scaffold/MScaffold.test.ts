import { h } from 'vue';
import { afterEach, describe, expect, it } from 'vitest';
import { enableAutoUnmount, mount } from '@vue/test-utils';
import MScaffold from './MScaffold.vue';

enableAutoUnmount(afterEach);

describe('scaffold', () => {
  it('should have header', () => {
    const wrapper = mount(MScaffold, {
      slots: {
        header: () => h('div', { id: 'header-slot' })
      }
    });

    expect(wrapper.find('.m-scaffold-header').exists()).toBe(true);
    expect(wrapper.find('#header-slot').exists()).toBe(true);
  });

  it('should not have header', () => {
    const wrapper = mount(MScaffold, {
      slots: {
        default: () => h('div', 'Default slot')
      }
    });

    expect(wrapper.find('.m-scaffold-header').exists()).toBe(false);
  });

  it('should have footer', () => {
    const wrapper = mount(MScaffold, {
      slots: {
        footer: () => h('div', { id: 'footer-slot' })
      }
    });

    expect(wrapper.find('.m-scaffold-footer').exists()).toBe(true);
    expect(wrapper.find('#footer-slot').exists()).toBe(true);
  });

  it('should not have footer', () => {
    const wrapper = mount(MScaffold, {
      slots: {
        default: () => h('div', 'Default slot')
      }
    });

    expect(wrapper.find('.m-scaffold-footer').exists()).toBe(false);
  });
});
