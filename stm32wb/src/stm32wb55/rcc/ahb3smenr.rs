#[doc = "Register `AHB3SMENR` reader"]
pub struct R(crate::R<AHB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3SMENR` writer"]
pub struct W(crate::W<AHB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3SMENR_SPEC>;
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
impl From<crate::W<AHB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSPISMEN` reader - QSPISMEN"]
pub type QSPISMEN_R = crate::BitReader<bool>;
#[doc = "Field `QSPISMEN` writer - QSPISMEN"]
pub type QSPISMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `PKASMEN` reader - PKA accelerator clocks enable during CPU1 sleep mode"]
pub type PKASMEN_R = crate::BitReader<bool>;
#[doc = "Field `PKASMEN` writer - PKA accelerator clocks enable during CPU1 sleep mode"]
pub type PKASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `AES2SMEN` reader - AES2 accelerator clocks enable during CPU1 sleep mode"]
pub type AES2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `AES2SMEN` writer - AES2 accelerator clocks enable during CPU1 sleep mode"]
pub type AES2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `RNGSMEN` reader - True RNG clocks enable during CPU1 sleep mode"]
pub type RNGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGSMEN` writer - True RNG clocks enable during CPU1 sleep mode"]
pub type RNGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode"]
pub type SRAM2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2SMEN` writer - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode"]
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `FLASHSMEN` reader - Flash interface clocks enable during CPU1 sleep mode"]
pub type FLASHSMEN_R = crate::BitReader<bool>;
#[doc = "Field `FLASHSMEN` writer - Flash interface clocks enable during CPU1 sleep mode"]
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - QSPISMEN"]
    #[inline(always)]
    pub fn qspismen(&self) -> QSPISMEN_R {
        QSPISMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - PKA accelerator clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AES2 accelerator clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    pub fn aes2smen(&self) -> AES2SMEN_R {
        AES2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - QSPISMEN"]
    #[inline(always)]
    #[must_use]
    pub fn qspismen(&mut self) -> QSPISMEN_W<8> {
        QSPISMEN_W::new(self)
    }
    #[doc = "Bit 16 - PKA accelerator clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pkasmen(&mut self) -> PKASMEN_W<16> {
        PKASMEN_W::new(self)
    }
    #[doc = "Bit 17 - AES2 accelerator clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes2smen(&mut self) -> AES2SMEN_W<17> {
        AES2SMEN_W::new(self)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<18> {
        RNGSMEN_W::new(self)
    }
    #[doc = "Bit 24 - SRAM2a and SRAM2b memory interface clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<24> {
        SRAM2SMEN_W::new(self)
    }
    #[doc = "Bit 25 - Flash interface clocks enable during CPU1 sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<25> {
        FLASHSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3smenr](index.html) module"]
pub struct AHB3SMENR_SPEC;
impl crate::RegisterSpec for AHB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3smenr::R](R) reader structure"]
impl crate::Readable for AHB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3smenr::W](W) writer structure"]
impl crate::Writable for AHB3SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3SMENR to value 0x0307_0100"]
impl crate::Resettable for AHB3SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0307_0100;
}
