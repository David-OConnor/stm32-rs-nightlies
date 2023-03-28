#[doc = "Register `RCC_AHB3RSTCLRR` reader"]
pub struct R(crate::R<RCC_AHB3RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB3RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB3RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB3RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB3RSTCLRR` writer"]
pub struct W(crate::W<RCC_AHB3RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB3RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB3RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB3RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMIRST` reader - DCMIRST"]
pub type DCMIRST_R = crate::BitReader<bool>;
#[doc = "Field `DCMIRST` writer - DCMIRST"]
pub type DCMIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `CRYP2RST` reader - CRYP2RST"]
pub type CRYP2RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYP2RST` writer - CRYP2RST"]
pub type CRYP2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `HASH2RST` reader - HASH2RST"]
pub type HASH2RST_R = crate::BitReader<bool>;
#[doc = "Field `HASH2RST` writer - HASH2RST"]
pub type HASH2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `RNG2RST` reader - RNG2RST"]
pub type RNG2RST_R = crate::BitReader<bool>;
#[doc = "Field `RNG2RST` writer - RNG2RST"]
pub type RNG2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `CRC2RST` reader - CRC2RST"]
pub type CRC2RST_R = crate::BitReader<bool>;
#[doc = "Field `CRC2RST` writer - CRC2RST"]
pub type CRC2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `HSEMRST` reader - HSEMRST"]
pub type HSEMRST_R = crate::BitReader<bool>;
#[doc = "Field `HSEMRST` writer - HSEMRST"]
pub type HSEMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
#[doc = "Field `IPCCRST` reader - IPCCRST"]
pub type IPCCRST_R = crate::BitReader<bool>;
#[doc = "Field `IPCCRST` writer - IPCCRST"]
pub type IPCCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB3RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DCMIRST"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP2RST"]
    #[inline(always)]
    pub fn cryp2rst(&self) -> CRYP2RST_R {
        CRYP2RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH2RST"]
    #[inline(always)]
    pub fn hash2rst(&self) -> HASH2RST_R {
        HASH2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG2RST"]
    #[inline(always)]
    pub fn rng2rst(&self) -> RNG2RST_R {
        RNG2RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC2RST"]
    #[inline(always)]
    pub fn crc2rst(&self) -> CRC2RST_R {
        CRC2RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - HSEMRST"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IPCCRST"]
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMIRST"]
    #[inline(always)]
    #[must_use]
    pub fn dcmirst(&mut self) -> DCMIRST_W<0> {
        DCMIRST_W::new(self)
    }
    #[doc = "Bit 4 - CRYP2RST"]
    #[inline(always)]
    #[must_use]
    pub fn cryp2rst(&mut self) -> CRYP2RST_W<4> {
        CRYP2RST_W::new(self)
    }
    #[doc = "Bit 5 - HASH2RST"]
    #[inline(always)]
    #[must_use]
    pub fn hash2rst(&mut self) -> HASH2RST_W<5> {
        HASH2RST_W::new(self)
    }
    #[doc = "Bit 6 - RNG2RST"]
    #[inline(always)]
    #[must_use]
    pub fn rng2rst(&mut self) -> RNG2RST_W<6> {
        RNG2RST_W::new(self)
    }
    #[doc = "Bit 7 - CRC2RST"]
    #[inline(always)]
    #[must_use]
    pub fn crc2rst(&mut self) -> CRC2RST_W<7> {
        CRC2RST_W::new(self)
    }
    #[doc = "Bit 11 - HSEMRST"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<11> {
        HSEMRST_W::new(self)
    }
    #[doc = "Bit 12 - IPCCRST"]
    #[inline(always)]
    #[must_use]
    pub fn ipccrst(&mut self) -> IPCCRST_W<12> {
        IPCCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb3rstclrr](index.html) module"]
pub struct RCC_AHB3RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB3RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb3rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_AHB3RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb3rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_AHB3RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB3RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB3RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
