import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'Oracle Stock Price Dashboard',
  description: 'Real-time visualization of stock price feeds and oracle status',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
