#[doc = "Register `SDMMC_DCTRL` reader"]
pub struct R(crate::R<SDMMC_DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_DCTRL` writer"]
pub struct W(crate::W<SDMMC_DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_DCTRL_SPEC>;
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
impl From<crate::W<SDMMC_DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTEN"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - DTEN"]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `DTDIR` reader - DTDIR"]
pub type DTDIR_R = crate::BitReader<bool>;
#[doc = "Field `DTDIR` writer - DTDIR"]
pub type DTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `DTMODE` reader - DTMODE"]
pub type DTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTMODE` writer - DTMODE"]
pub type DTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMMC_DCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DBLOCKSIZE` reader - DBLOCKSIZE"]
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLOCKSIZE` writer - DBLOCKSIZE"]
pub type DBLOCKSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_DCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RWSTART` reader - RWSTART"]
pub type RWSTART_R = crate::BitReader<bool>;
#[doc = "Field `RWSTART` writer - RWSTART"]
pub type RWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `RWSTOP` reader - RWSTOP"]
pub type RWSTOP_R = crate::BitReader<bool>;
#[doc = "Field `RWSTOP` writer - RWSTOP"]
pub type RWSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `RWMOD` reader - RWMOD"]
pub type RWMOD_R = crate::BitReader<bool>;
#[doc = "Field `RWMOD` writer - RWMOD"]
pub type RWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub type SDIOEN_R = crate::BitReader<bool>;
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `BOOTACKEN` reader - BOOTACKEN"]
pub type BOOTACKEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOTACKEN` writer - BOOTACKEN"]
pub type BOOTACKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
#[doc = "Field `FIFORST` reader - FIFORST"]
pub type FIFORST_R = crate::BitReader<bool>;
#[doc = "Field `FIFORST` writer - FIFORST"]
pub type FIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_DCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - RWSTART"]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RWSTOP"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOTACKEN"]
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FIFORST"]
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<1> {
        DTDIR_W::new(self)
    }
    #[doc = "Bits 2:3 - DTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<2> {
        DTMODE_W::new(self)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<4> {
        DBLOCKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - RWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<8> {
        RWSTART_W::new(self)
    }
    #[doc = "Bit 9 - RWSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<9> {
        RWSTOP_W::new(self)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<10> {
        RWMOD_W::new(self)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    #[doc = "Bit 12 - BOOTACKEN"]
    #[inline(always)]
    #[must_use]
    pub fn bootacken(&mut self) -> BOOTACKEN_W<12> {
        BOOTACKEN_W::new(self)
    }
    #[doc = "Bit 13 - FIFORST"]
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FIFORST_W<13> {
        FIFORST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dctrl](index.html) module"]
pub struct SDMMC_DCTRL_SPEC;
impl crate::RegisterSpec for SDMMC_DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dctrl::R](R) reader structure"]
impl crate::Readable for SDMMC_DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_dctrl::W](W) writer structure"]
impl crate::Writable for SDMMC_DCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_DCTRL to value 0"]
impl crate::Resettable for SDMMC_DCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
