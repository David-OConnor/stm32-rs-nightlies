#[doc = "Register `OTYPER` reader"]
pub struct R(crate::R<OTYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTYPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTYPER` writer"]
pub struct W(crate::W<OTYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTYPER_SPEC>;
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
impl From<crate::W<OTYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTYPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OT0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT0_R = crate::BitReader<bool>;
#[doc = "Field `OT0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT1_R = crate::BitReader<bool>;
#[doc = "Field `OT1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT2_R = crate::BitReader<bool>;
#[doc = "Field `OT2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT3_R = crate::BitReader<bool>;
#[doc = "Field `OT3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT4_R = crate::BitReader<bool>;
#[doc = "Field `OT4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT5_R = crate::BitReader<bool>;
#[doc = "Field `OT5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT6_R = crate::BitReader<bool>;
#[doc = "Field `OT6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT7_R = crate::BitReader<bool>;
#[doc = "Field `OT7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT8_R = crate::BitReader<bool>;
#[doc = "Field `OT8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT9_R = crate::BitReader<bool>;
#[doc = "Field `OT9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT10_R = crate::BitReader<bool>;
#[doc = "Field `OT10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT11_R = crate::BitReader<bool>;
#[doc = "Field `OT11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT12_R = crate::BitReader<bool>;
#[doc = "Field `OT12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT13_R = crate::BitReader<bool>;
#[doc = "Field `OT13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT14_R = crate::BitReader<bool>;
#[doc = "Field `OT14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
#[doc = "Field `OT15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT15_R = crate::BitReader<bool>;
#[doc = "Field `OT15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot7(&self) -> OT7_R {
        OT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot8(&self) -> OT8_R {
        OT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot9(&self) -> OT9_R {
        OT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot10(&self) -> OT10_R {
        OT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot11(&self) -> OT11_R {
        OT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot12(&self) -> OT12_R {
        OT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> OT0_W<0> {
        OT0_W::new(self)
    }
    #[doc = "Bit 1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> OT1_W<1> {
        OT1_W::new(self)
    }
    #[doc = "Bit 2 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> OT2_W<2> {
        OT2_W::new(self)
    }
    #[doc = "Bit 3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<3> {
        OT3_W::new(self)
    }
    #[doc = "Bit 4 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> OT4_W<4> {
        OT4_W::new(self)
    }
    #[doc = "Bit 5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot5(&mut self) -> OT5_W<5> {
        OT5_W::new(self)
    }
    #[doc = "Bit 6 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot6(&mut self) -> OT6_W<6> {
        OT6_W::new(self)
    }
    #[doc = "Bit 7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot7(&mut self) -> OT7_W<7> {
        OT7_W::new(self)
    }
    #[doc = "Bit 8 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot8(&mut self) -> OT8_W<8> {
        OT8_W::new(self)
    }
    #[doc = "Bit 9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot9(&mut self) -> OT9_W<9> {
        OT9_W::new(self)
    }
    #[doc = "Bit 10 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot10(&mut self) -> OT10_W<10> {
        OT10_W::new(self)
    }
    #[doc = "Bit 11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot11(&mut self) -> OT11_W<11> {
        OT11_W::new(self)
    }
    #[doc = "Bit 12 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot12(&mut self) -> OT12_W<12> {
        OT12_W::new(self)
    }
    #[doc = "Bit 13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot13(&mut self) -> OT13_W<13> {
        OT13_W::new(self)
    }
    #[doc = "Bit 14 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot14(&mut self) -> OT14_W<14> {
        OT14_W::new(self)
    }
    #[doc = "Bit 15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    #[must_use]
    pub fn ot15(&mut self) -> OT15_W<15> {
        OT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](index.html) module"]
pub struct OTYPER_SPEC;
impl crate::RegisterSpec for OTYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otyper::R](R) reader structure"]
impl crate::Readable for OTYPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otyper::W](W) writer structure"]
impl crate::Writable for OTYPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OTYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
