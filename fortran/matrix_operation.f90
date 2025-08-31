program matrix_mul
  implicit none
  integer, parameter :: dp = kind(1.0d0)
  integer, parameter :: n = 1000  ! Use 1000+ for benchmarks

  real(dp), allocatable :: a(:,:), b(:,:), c(:,:)
  integer :: i, j, k
  real(dp) :: start, finish, checksum

  allocate(a(n,n), b(n,n), c(n,n))

  ! Fill matrices with known values
  do i = 1, n
     do j = 1, n
        a(i,j) = mod((i - 1) * n + (j - 1), 100)
        b(i,j) = mod((i - 1) * n + (j - 1), 100)
     end do
  end do

  ! Print inputs (optional for small n)
  ! if (n <= 5) then
  !    print *, "Matrix A:"
  !    call print_matrix(a, n)
  !    print *, "Matrix B:"
  !    call print_matrix(b, n)
  ! end if

  ! Time only the matrix multiplication
  call cpu_time(start)
  do i = 1, n
     do j = 1, n
        c(i,j) = 0.0_dp
        do k = 1, n
           c(i,j) = c(i,j) + a(i,k) * b(k,j)
        end do
     end do
  end do
  call cpu_time(finish)

  ! Output
  print *, "Matrix size:", n, "x", n
  print *, "Time taken:", finish - start, "seconds"

  ! if (n <= 5) then
  !    print *, "Result matrix C:"
  !    call print_matrix(c, n)
  ! end if

  checksum = sum(c)
  print *, "Result checksum:", checksum

contains

  subroutine print_matrix(mat, n)
    real(dp), intent(in) :: mat(n,n)
    integer, intent(in) :: n
    integer :: i, j
    do i = 1, n
       write(*,'(100(f8.2,1x))') (mat(i,j), j=1,n)
    end do
  end subroutine print_matrix

end program matrix_mul

