import { render, screen } from '@testing-library/react';
import Home from './page';

describe('Home Page', () => {
  it('renders the home page', () => {
    render(<Home />);
    expect(screen.getByText(/Oracle Stock Price Dashboard/i)).toBeInTheDocument();
  });
});
