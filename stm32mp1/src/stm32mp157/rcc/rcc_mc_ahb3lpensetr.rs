#[doc = "Register `RCC_MC_AHB3LPENSETR` reader"]
pub struct R(crate::R<RCC_MC_AHB3LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB3LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB3LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB3LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB3LPENSETR` writer"]
pub struct W(crate::W<RCC_MC_AHB3LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB3LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB3LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB3LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMILPEN` reader - DCMILPEN"]
pub type DCMILPEN_R = crate::BitReader<bool>;
#[doc = "Field `DCMILPEN` writer - DCMILPEN"]
pub type DCMILPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
#[doc = "Field `CRYP2LPEN` reader - CRYP2LPEN"]
pub type CRYP2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `CRYP2LPEN` writer - CRYP2LPEN"]
pub type CRYP2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
#[doc = "Field `HASH2LPEN` reader - HASH2LPEN"]
pub type HASH2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `HASH2LPEN` writer - HASH2LPEN"]
pub type HASH2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
#[doc = "Field `RNG2LPEN` reader - RNG2LPEN"]
pub type RNG2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `RNG2LPEN` writer - RNG2LPEN"]
pub type RNG2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
#[doc = "Field `CRC2LPEN` reader - CRC2LPEN"]
pub type CRC2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `CRC2LPEN` writer - CRC2LPEN"]
pub type CRC2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
#[doc = "Field `HSEMLPEN` reader - HSEMLPEN"]
pub type HSEMLPEN_R = crate::BitReader<bool>;
#[doc = "Field `HSEMLPEN` writer - HSEMLPEN"]
pub type HSEMLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
#[doc = "Field `IPCCLPEN` reader - IPCCLPEN"]
pub type IPCCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `IPCCLPEN` writer - IPCCLPEN"]
pub type IPCCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    pub fn cryp2lpen(&self) -> CRYP2LPEN_R {
        CRYP2LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    pub fn hash2lpen(&self) -> HASH2LPEN_R {
        HASH2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    pub fn rng2lpen(&self) -> RNG2LPEN_R {
        RNG2LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    pub fn crc2lpen(&self) -> CRC2LPEN_R {
        CRC2LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    pub fn hsemlpen(&self) -> HSEMLPEN_R {
        HSEMLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    pub fn ipcclpen(&self) -> IPCCLPEN_R {
        IPCCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<0> {
        DCMILPEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cryp2lpen(&mut self) -> CRYP2LPEN_W<4> {
        CRYP2LPEN_W::new(self)
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hash2lpen(&mut self) -> HASH2LPEN_W<5> {
        HASH2LPEN_W::new(self)
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn rng2lpen(&mut self) -> RNG2LPEN_W<6> {
        RNG2LPEN_W::new(self)
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    #[must_use]
    pub fn crc2lpen(&mut self) -> CRC2LPEN_W<7> {
        CRC2LPEN_W::new(self)
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsemlpen(&mut self) -> HSEMLPEN_W<11> {
        HSEMLPEN_W::new(self)
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ipcclpen(&mut self) -> IPCCLPEN_W<12> {
        IPCCLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb3lpensetr](index.html) module"]
pub struct RCC_MC_AHB3LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB3LPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb3lpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB3LPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb3lpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB3LPENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB3LPENSETR to value 0x18f1"]
impl crate::Resettable for RCC_MC_AHB3LPENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18f1;
}
