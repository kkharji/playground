#import <Foundation/Foundation.h>


int main (int argc, const char * argv[]) {
  // Cocoa’s reference-counted memory management system.
  // Acts as a catch-all for allocations, cleaning up memory when objects are no longer needed.
  // FYI: more efferently wrap with @autoreleasepool {}
  //
  // This object contains objects that have received an autorelease message and
  // when drained it sends a release message to each of those objects.
  // In a garbage-collected environment, there is no need for autorelease
  // pools.
  //
  // Often see autorelease in situations where you have alloced an object in a
  // method and you want to return that object.
  NSAutoreleasePool *pool = [[NSAutoreleasePool alloc] init];

  // Like printf. The leading @ signifies to the compiler that this is an NSString.
  NSLog(@"Hello World");

  [pool drain];

  return 0;
}
