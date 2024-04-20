// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: person.proto

#ifndef GOOGLE_PROTOBUF_INCLUDED_person_2eproto
#define GOOGLE_PROTOBUF_INCLUDED_person_2eproto

#include <limits>
#include <string>

#include <google/protobuf/port_def.inc>
#if PROTOBUF_VERSION < 3013000
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers. Please update
#error your headers.
#endif
#if 3013000 < PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers. Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/port_undef.inc>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_table_driven.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/inlined_string_field.h>
#include <google/protobuf/metadata_lite.h>
#include <google/protobuf/generated_message_reflection.h>
#include <google/protobuf/message.h>
#include <google/protobuf/repeated_field.h>  // IWYU pragma: export
#include <google/protobuf/extension_set.h>  // IWYU pragma: export
#include <google/protobuf/unknown_field_set.h>
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>
#define PROTOBUF_INTERNAL_EXPORT_person_2eproto
PROTOBUF_NAMESPACE_OPEN
namespace internal {
class AnyMetadata;
}  // namespace internal
PROTOBUF_NAMESPACE_CLOSE

// Internal implementation detail -- do not use these members.
struct TableStruct_person_2eproto {
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTableField entries[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::AuxiliaryParseTableField aux[]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::ParseTable schema[1]
    PROTOBUF_SECTION_VARIABLE(protodesc_cold);
  static const ::PROTOBUF_NAMESPACE_ID::internal::FieldMetadata field_metadata[];
  static const ::PROTOBUF_NAMESPACE_ID::internal::SerializationTable serialization_table[];
  static const ::PROTOBUF_NAMESPACE_ID::uint32 offsets[];
};
extern const ::PROTOBUF_NAMESPACE_ID::internal::DescriptorTable descriptor_table_person_2eproto;
class Person;
class PersonDefaultTypeInternal;
extern PersonDefaultTypeInternal _Person_default_instance_;
PROTOBUF_NAMESPACE_OPEN
template<> ::Person* Arena::CreateMaybeMessage<::Person>(Arena*);
PROTOBUF_NAMESPACE_CLOSE

// ===================================================================

class Person PROTOBUF_FINAL :
    public ::PROTOBUF_NAMESPACE_ID::Message /* @@protoc_insertion_point(class_definition:Person) */ {
 public:
  inline Person() : Person(nullptr) {}
  virtual ~Person();

  Person(const Person& from);
  Person(Person&& from) noexcept
    : Person() {
    *this = ::std::move(from);
  }

  inline Person& operator=(const Person& from) {
    CopyFrom(from);
    return *this;
  }
  inline Person& operator=(Person&& from) noexcept {
    if (GetArena() == from.GetArena()) {
      if (this != &from) InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* GetDescriptor() {
    return GetMetadataStatic().descriptor;
  }
  static const ::PROTOBUF_NAMESPACE_ID::Reflection* GetReflection() {
    return GetMetadataStatic().reflection;
  }
  static const Person& default_instance();

  static void InitAsDefaultInstance();  // FOR INTERNAL USE ONLY
  static inline const Person* internal_default_instance() {
    return reinterpret_cast<const Person*>(
               &_Person_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(Person& a, Person& b) {
    a.Swap(&b);
  }
  inline void Swap(Person* other) {
    if (other == this) return;
    if (GetArena() == other->GetArena()) {
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(Person* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetArena() == other->GetArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  inline Person* New() const final {
    return CreateMaybeMessage<Person>(nullptr);
  }

  Person* New(::PROTOBUF_NAMESPACE_ID::Arena* arena) const final {
    return CreateMaybeMessage<Person>(arena);
  }
  void CopyFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) final;
  void MergeFrom(const ::PROTOBUF_NAMESPACE_ID::Message& from) final;
  void CopyFrom(const Person& from);
  void MergeFrom(const Person& from);
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  ::PROTOBUF_NAMESPACE_ID::uint8* _InternalSerialize(
      ::PROTOBUF_NAMESPACE_ID::uint8* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _cached_size_.Get(); }

  private:
  inline void SharedCtor();
  inline void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(Person* other);
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "Person";
  }
  protected:
  explicit Person(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  private:
  static void ArenaDtor(void* object);
  inline void RegisterArenaDtor(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  public:

  ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadata() const final;
  private:
  static ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadataStatic() {
    ::PROTOBUF_NAMESPACE_ID::internal::AssignDescriptors(&::descriptor_table_person_2eproto);
    return ::descriptor_table_person_2eproto.file_level_metadata[kIndexInFileMessages];
  }

  public:

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kQueryFieldNumber = 1,
    kPageNumberFieldNumber = 2,
    kPageSizeFieldNumber = 3,
  };
  // string query = 1;
  void clear_query();
  const std::string& query() const;
  void set_query(const std::string& value);
  void set_query(std::string&& value);
  void set_query(const char* value);
  void set_query(const char* value, size_t size);
  std::string* mutable_query();
  std::string* release_query();
  void set_allocated_query(std::string* query);
  private:
  const std::string& _internal_query() const;
  void _internal_set_query(const std::string& value);
  std::string* _internal_mutable_query();
  public:

  // int32 page_number = 2;
  void clear_page_number();
  ::PROTOBUF_NAMESPACE_ID::int32 page_number() const;
  void set_page_number(::PROTOBUF_NAMESPACE_ID::int32 value);
  private:
  ::PROTOBUF_NAMESPACE_ID::int32 _internal_page_number() const;
  void _internal_set_page_number(::PROTOBUF_NAMESPACE_ID::int32 value);
  public:

  // int32 page_size = 3;
  void clear_page_size();
  ::PROTOBUF_NAMESPACE_ID::int32 page_size() const;
  void set_page_size(::PROTOBUF_NAMESPACE_ID::int32 value);
  private:
  ::PROTOBUF_NAMESPACE_ID::int32 _internal_page_size() const;
  void _internal_set_page_size(::PROTOBUF_NAMESPACE_ID::int32 value);
  public:

  // @@protoc_insertion_point(class_scope:Person)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr query_;
  ::PROTOBUF_NAMESPACE_ID::int32 page_number_;
  ::PROTOBUF_NAMESPACE_ID::int32 page_size_;
  mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  friend struct ::TableStruct_person_2eproto;
};
// ===================================================================


// ===================================================================

#ifdef __GNUC__
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// Person

// string query = 1;
inline void Person::clear_query() {
  query_.ClearToEmpty(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline const std::string& Person::query() const {
  // @@protoc_insertion_point(field_get:Person.query)
  return _internal_query();
}
inline void Person::set_query(const std::string& value) {
  _internal_set_query(value);
  // @@protoc_insertion_point(field_set:Person.query)
}
inline std::string* Person::mutable_query() {
  // @@protoc_insertion_point(field_mutable:Person.query)
  return _internal_mutable_query();
}
inline const std::string& Person::_internal_query() const {
  return query_.Get();
}
inline void Person::_internal_set_query(const std::string& value) {
  
  query_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), value, GetArena());
}
inline void Person::set_query(std::string&& value) {
  
  query_.Set(
    &::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::move(value), GetArena());
  // @@protoc_insertion_point(field_set_rvalue:Person.query)
}
inline void Person::set_query(const char* value) {
  GOOGLE_DCHECK(value != nullptr);
  
  query_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(value),
              GetArena());
  // @@protoc_insertion_point(field_set_char:Person.query)
}
inline void Person::set_query(const char* value,
    size_t size) {
  
  query_.Set(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), ::std::string(
      reinterpret_cast<const char*>(value), size), GetArena());
  // @@protoc_insertion_point(field_set_pointer:Person.query)
}
inline std::string* Person::_internal_mutable_query() {
  
  return query_.Mutable(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline std::string* Person::release_query() {
  // @@protoc_insertion_point(field_release:Person.query)
  return query_.Release(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), GetArena());
}
inline void Person::set_allocated_query(std::string* query) {
  if (query != nullptr) {
    
  } else {
    
  }
  query_.SetAllocated(&::PROTOBUF_NAMESPACE_ID::internal::GetEmptyStringAlreadyInited(), query,
      GetArena());
  // @@protoc_insertion_point(field_set_allocated:Person.query)
}

// int32 page_number = 2;
inline void Person::clear_page_number() {
  page_number_ = 0;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Person::_internal_page_number() const {
  return page_number_;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Person::page_number() const {
  // @@protoc_insertion_point(field_get:Person.page_number)
  return _internal_page_number();
}
inline void Person::_internal_set_page_number(::PROTOBUF_NAMESPACE_ID::int32 value) {
  
  page_number_ = value;
}
inline void Person::set_page_number(::PROTOBUF_NAMESPACE_ID::int32 value) {
  _internal_set_page_number(value);
  // @@protoc_insertion_point(field_set:Person.page_number)
}

// int32 page_size = 3;
inline void Person::clear_page_size() {
  page_size_ = 0;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Person::_internal_page_size() const {
  return page_size_;
}
inline ::PROTOBUF_NAMESPACE_ID::int32 Person::page_size() const {
  // @@protoc_insertion_point(field_get:Person.page_size)
  return _internal_page_size();
}
inline void Person::_internal_set_page_size(::PROTOBUF_NAMESPACE_ID::int32 value) {
  
  page_size_ = value;
}
inline void Person::set_page_size(::PROTOBUF_NAMESPACE_ID::int32 value) {
  _internal_set_page_size(value);
  // @@protoc_insertion_point(field_set:Person.page_size)
}

#ifdef __GNUC__
  #pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)


// @@protoc_insertion_point(global_scope)

#include <google/protobuf/port_undef.inc>
#endif  // GOOGLE_PROTOBUF_INCLUDED_GOOGLE_PROTOBUF_INCLUDED_person_2eproto
