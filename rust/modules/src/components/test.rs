pub fn do_thing_test() {
    println!("In file included from /usr/include/c++/4.6/algorithm:63:0,
    from error_code.cpp:2:
/usr/include/c++/4.6/bits/stl_algo.h: In function ‘_RandomAccessIterator std::__find(_RandomAccessIterator, _RandomAccessIterator, const _Tp&, std::random_access_iterator_tag) [with _RandomAccessIterator = __gnu_cxx::__normal_iterator*, std::vector > >, _Tp = int]’:
/usr/include/c++/4.6/bits/stl_algo.h:4403:45:   instantiated from ‘_IIter std::find(_IIter, _IIter, const _Tp&) [with _IIter = __gnu_cxx::__normal_iterator*, std::vector > >, _Tp = int]’
error_code.cpp:8:89:   instantiated from here
/usr/include/c++/4.6/bits/stl_algo.h:162:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:162:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:799:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_algo.h:4403:45:   instantiated from ‘_IIter std::find(_IIter, _IIter, const _Tp&) [with _IIter = __gnu_cxx::__normal_iterator*, std::vector > >, _Tp = int]’
error_code.cpp:8:89:   instantiated from here
/usr/include/c++/4.6/bits/stl_algo.h:166:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:166:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:799:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_algo.h:170:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:170:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:799:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_algo.h:174:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:174:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:799:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_algo.h:182:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:182:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:799:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_algo.h:186:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:186:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:799:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)
/usr/include/c++/4.6/bits/stl_algo.h:190:4: error: no match for ‘operator==’ in ‘__first.__gnu_cxx::__normal_iterator::operator* [with _Iterator = std::vector*, _Container = std::vector >, __gnu_cxx::__normal_iterator::reference = std::vector&]() == __val’
/usr/include/c++/4.6/bits/stl_algo.h:190:4: note: candidates are:
/usr/include/c++/4.6/bits/stl_pair.h:201:5: note: template bool std::operator==(const std::pair&, const std::pair&)
/usr/include/c++/4.6/bits/stl_iterator.h:285:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/stl_iterator.h:335:5: note: template bool std::operator==(const std::reverse_iterator&, const std::reverse_iterator&)
/usr/include/c++/4.6/bits/allocator.h:122:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/allocator.h:127:5: note: template bool std::operator==(const std::allocator&, const std::allocator&)
/usr/include/c++/4.6/bits/stl_vector.h:1273:5: note: template bool std::operator==(const std::vector&, const std::vector&)
/usr/include/c++/4.6/ext/new_allocator.h:123:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::new_allocator&, const __gnu_cxx::new_allocator&)
/usr/include/c++/4.6/bits/stl_iterator.h:805:5: note: template bool __gnu_cxx::operator==(const __gnu_cxx::__normal_iterator&, const __gnu_cxx::__normal_iterator&)")
}
