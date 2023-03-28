#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR0` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR1` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR2` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR3` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR4` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR5` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR6` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR7` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR8` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR9` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR10` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR11` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR12` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR13` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR14` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR15` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<15> {
        BR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
