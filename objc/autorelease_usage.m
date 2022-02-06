#import <Foundation/Foundation.h>
@interface MyObj: NSObject
// a method that is automatically called when the object get deallocated.
-(void)dealloc;
@end

@implementation MyObj
-(void)dealloc {
  printf("///////////////////////Deallocated the Object.\n");

  // Calling the original method
  [super dealloc]; // warns with -Wobjc-missing-super-calls
}
@end

void leakingPool() {
  NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];
  MyObj *myObj = [[MyObj alloc] init];

  // Add to this function's pool...
  [myObj autorelease];
  // ... and forget to release the pool! *Gasp*
}

void containedPool()
{
  NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];
  MyObj *myObj = [[MyObj alloc] init];

  // Add object to be auto release
  [myObj autorelease];

  // ... and release the pool!
  [pool release];
}

int main() {
  NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

  printf("Creating a contained pool --------------------\n");
  containedPool();

  printf("Creating a leaky pool ------------------------\n");
  leakingPool();

  printf("Deallocating current scope pool --------------\n");
  [pool release];

  return 0;
}
