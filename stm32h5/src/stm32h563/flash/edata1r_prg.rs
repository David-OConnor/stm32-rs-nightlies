#[doc = "Register `EDATA1R_PRG` reader"]
pub struct R(crate::R<EDATA1R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDATA1R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDATA1R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDATA1R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDATA1R_PRG` writer"]
pub struct W(crate::W<EDATA1R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDATA1R_PRG_SPEC>;
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
impl From<crate::W<EDATA1R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDATA1R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDATA1_STRT` reader - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
pub type EDATA1_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDATA1_STRT` writer - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
pub type EDATA1_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EDATA1R_PRG_SPEC, u8, u8, 3, O>;
#[doc = "Field `EDATA1_EN` reader - Bank 1 flash high-cycle data enable"]
pub type EDATA1_EN_R = crate::BitReader<bool>;
#[doc = "Field `EDATA1_EN` writer - Bank 1 flash high-cycle data enable"]
pub type EDATA1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EDATA1R_PRG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
    #[inline(always)]
    pub fn edata1_strt(&self) -> EDATA1_STRT_R {
        EDATA1_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 1 flash high-cycle data enable"]
    #[inline(always)]
    pub fn edata1_en(&self) -> EDATA1_EN_R {
        EDATA1_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - EDATA1_STRT contains the start sectors of the flash high-cycle data area in Bank 1 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 1 are reserved for flash high-cycle data"]
    #[inline(always)]
    #[must_use]
    pub fn edata1_strt(&mut self) -> EDATA1_STRT_W<0> {
        EDATA1_STRT_W::new(self)
    }
    #[doc = "Bit 15 - Bank 1 flash high-cycle data enable"]
    #[inline(always)]
    #[must_use]
    pub fn edata1_en(&mut self) -> EDATA1_EN_W<15> {
        EDATA1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH data sector configuration Bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edata1r_prg](index.html) module"]
pub struct EDATA1R_PRG_SPEC;
impl crate::RegisterSpec for EDATA1R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edata1r_prg::R](R) reader structure"]
impl crate::Readable for EDATA1R_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edata1r_prg::W](W) writer structure"]
impl crate::Writable for EDATA1R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDATA1R_PRG to value 0"]
impl crate::Resettable for EDATA1R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
