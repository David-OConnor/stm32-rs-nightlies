#[doc = "Register `OPTSR2_PRG` reader"]
pub struct R(crate::R<OPTSR2_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTSR2_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTSR2_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTSR2_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTSR2_PRG` writer"]
pub struct W(crate::W<OPTSR2_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTSR2_PRG_SPEC>;
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
impl From<crate::W<OPTSR2_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTSR2_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM2_RST` reader - SRAM2 erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2_RST` writer - SRAM2 erase when system reset"]
pub type SRAM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR2_PRG_SPEC, bool, O>;
#[doc = "Field `BKPRAM_ECC` reader - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_R = crate::BitReader<bool>;
#[doc = "Field `BKPRAM_ECC` writer - Backup RAM ECC detection and correction disable"]
pub type BKPRAM_ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR2_PRG_SPEC, bool, O>;
#[doc = "Field `SRAM2_ECC` reader - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2_ECC` writer - SRAM2 ECC detection and correction disable"]
pub type SRAM2_ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR2_PRG_SPEC, bool, O>;
#[doc = "Field `SRAM1_RST` reader - SRAM1 erase upon system reset"]
pub type SRAM1_RST_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1_RST` writer - SRAM1 erase upon system reset"]
pub type SRAM1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR2_PRG_SPEC, bool, O>;
#[doc = "Field `SRAM1_ECC` reader - SRAM1 ECC detection and correction disable"]
pub type SRAM1_ECC_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1_ECC` writer - SRAM1 ECC detection and correction disable"]
pub type SRAM1_ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTSR2_PRG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - SRAM2 erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    pub fn bkpram_ecc(&self) -> BKPRAM_ECC_R {
        BKPRAM_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    pub fn sram2_ecc(&self) -> SRAM2_ECC_R {
        SRAM2_ECC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM1 erase upon system reset"]
    #[inline(always)]
    pub fn sram1_rst(&self) -> SRAM1_RST_R {
        SRAM1_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM1 ECC detection and correction disable"]
    #[inline(always)]
    pub fn sram1_ecc(&self) -> SRAM1_ECC_R {
        SRAM1_ECC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SRAM2 erase when system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<3> {
        SRAM2_RST_W::new(self)
    }
    #[doc = "Bit 4 - Backup RAM ECC detection and correction disable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpram_ecc(&mut self) -> BKPRAM_ECC_W<4> {
        BKPRAM_ECC_W::new(self)
    }
    #[doc = "Bit 6 - SRAM2 ECC detection and correction disable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2_ecc(&mut self) -> SRAM2_ECC_W<6> {
        SRAM2_ECC_W::new(self)
    }
    #[doc = "Bit 9 - SRAM1 erase upon system reset"]
    #[inline(always)]
    #[must_use]
    pub fn sram1_rst(&mut self) -> SRAM1_RST_W<9> {
        SRAM1_RST_W::new(self)
    }
    #[doc = "Bit 10 - SRAM1 ECC detection and correction disable"]
    #[inline(always)]
    #[must_use]
    pub fn sram1_ecc(&mut self) -> SRAM1_ECC_W<10> {
        SRAM1_ECC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option status register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optsr2_prg](index.html) module"]
pub struct OPTSR2_PRG_SPEC;
impl crate::RegisterSpec for OPTSR2_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optsr2_prg::R](R) reader structure"]
impl crate::Readable for OPTSR2_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optsr2_prg::W](W) writer structure"]
impl crate::Writable for OPTSR2_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTSR2_PRG to value 0"]
impl crate::Resettable for OPTSR2_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
