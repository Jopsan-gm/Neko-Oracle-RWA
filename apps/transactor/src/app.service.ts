import { Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  getStatus(): { status: string; timestamp: number } {
    return {
      status: 'ready',
      timestamp: Date.now(),
    };
  }
}
