/* Generated-by: [20260322-06-core-gateway-implementation] */
import { IContext, IMiddleware, NextFunction } from './types.js';

export class PipelineExecutor {
  private middlewares: IMiddleware[] = [];

  use(middleware: IMiddleware): void {
    this.middlewares.push(middleware);
  }

  async run(context: IContext): Promise<void> {
    let index = -1;

    const dispatch = async (i: number): Promise<void> => {
      if (i <= index) throw new Error('next() called multiple times');
      index = i;
      
      const middleware = this.middlewares[i];
      if (middleware) {
        await middleware.execute(context, () => dispatch(i + 1));
      }
    };

    await dispatch(0);
  }
}
