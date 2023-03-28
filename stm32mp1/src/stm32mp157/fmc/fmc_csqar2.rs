#[doc = "Register `FMC_CSQAR2` reader"]
pub struct R(crate::R<FMC_CSQAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CSQAR2` writer"]
pub struct W(crate::W<FMC_CSQAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CSQAR2_SPEC>;
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
impl From<crate::W<FMC_CSQAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CSQAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDC5` reader - ADDC5"]
pub type ADDC5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDC5` writer - ADDC5"]
pub type ADDC5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQAR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `NANDCEN0` reader - NANDCEN0"]
pub type NANDCEN0_R = crate::BitReader<bool>;
#[doc = "Field `NANDCEN0` writer - NANDCEN0"]
pub type NANDCEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQAR2_SPEC, bool, O>;
#[doc = "Field `NANDCEN1` reader - NANDCEN1"]
pub type NANDCEN1_R = crate::BitReader<bool>;
#[doc = "Field `NANDCEN1` writer - NANDCEN1"]
pub type NANDCEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_CSQAR2_SPEC, bool, O>;
#[doc = "Field `SAO` reader - SAO"]
pub type SAO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAO` writer - SAO"]
pub type SAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_CSQAR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    pub fn addc5(&self) -> ADDC5_R {
        ADDC5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    pub fn nandcen0(&self) -> NANDCEN0_R {
        NANDCEN0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    pub fn nandcen1(&self) -> NANDCEN1_R {
        NANDCEN1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADDC5"]
    #[inline(always)]
    #[must_use]
    pub fn addc5(&mut self) -> ADDC5_W<0> {
        ADDC5_W::new(self)
    }
    #[doc = "Bit 10 - NANDCEN0"]
    #[inline(always)]
    #[must_use]
    pub fn nandcen0(&mut self) -> NANDCEN0_W<10> {
        NANDCEN0_W::new(self)
    }
    #[doc = "Bit 11 - NANDCEN1"]
    #[inline(always)]
    #[must_use]
    pub fn nandcen1(&mut self) -> NANDCEN1_W<11> {
        NANDCEN1_W::new(self)
    }
    #[doc = "Bits 16:31 - SAO"]
    #[inline(always)]
    #[must_use]
    pub fn sao(&mut self) -> SAO_W<16> {
        SAO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqar2](index.html) module"]
pub struct FMC_CSQAR2_SPEC;
impl crate::RegisterSpec for FMC_CSQAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqar2::R](R) reader structure"]
impl crate::Readable for FMC_CSQAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_csqar2::W](W) writer structure"]
impl crate::Writable for FMC_CSQAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CSQAR2 to value 0x0002_0000"]
impl crate::Resettable for FMC_CSQAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0000;
}
