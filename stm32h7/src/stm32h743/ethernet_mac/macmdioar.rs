#[doc = "Register `MACMDIOAR` reader"]
pub struct R(crate::R<MACMDIOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMDIOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMDIOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMDIOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMDIOAR` writer"]
pub struct W(crate::W<MACMDIOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMDIOAR_SPEC>;
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
impl From<crate::W<MACMDIOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMDIOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - MII Busy"]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - MII Busy"]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `C45E` reader - Clause 45 PHY Enable"]
pub type C45E_R = crate::BitReader<bool>;
#[doc = "Field `C45E` writer - Clause 45 PHY Enable"]
pub type C45E_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `GOC` reader - MII Operation Command"]
pub type GOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GOC` writer - MII Operation Command"]
pub type GOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIOAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SKAP` reader - Skip Address Packet"]
pub type SKAP_R = crate::BitReader<bool>;
#[doc = "Field `SKAP` writer - Skip Address Packet"]
pub type SKAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `CR` reader - CSR Clock Range"]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - CSR Clock Range"]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIOAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `NTC` reader - Number of Training Clocks"]
pub type NTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTC` writer - Number of Training Clocks"]
pub type NTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIOAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RDA` reader - Register/Device Address"]
pub type RDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDA` writer - Register/Device Address"]
pub type RDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIOAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - Physical Layer Address"]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - Physical Layer Address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMDIOAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BTB` reader - Back to Back transactions"]
pub type BTB_R = crate::BitReader<bool>;
#[doc = "Field `BTB` writer - Back to Back transactions"]
pub type BTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `PSE` reader - Preamble Suppression Enable"]
pub type PSE_R = crate::BitReader<bool>;
#[doc = "Field `PSE` writer - Preamble Suppression Enable"]
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACMDIOAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Skip Address Packet"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<0> {
        MB_W::new(self)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<1> {
        C45E_W::new(self)
    }
    #[doc = "Bits 2:3 - MII Operation Command"]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<2> {
        GOC_W::new(self)
    }
    #[doc = "Bit 4 - Skip Address Packet"]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<4> {
        SKAP_W::new(self)
    }
    #[doc = "Bits 8:11 - CSR Clock Range"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<8> {
        CR_W::new(self)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<12> {
        NTC_W::new(self)
    }
    #[doc = "Bits 16:20 - Register/Device Address"]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<16> {
        RDA_W::new(self)
    }
    #[doc = "Bits 21:25 - Physical Layer Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<21> {
        PA_W::new(self)
    }
    #[doc = "Bit 26 - Back to Back transactions"]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<26> {
        BTB_W::new(self)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<27> {
        PSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmdioar](index.html) module"]
pub struct MACMDIOAR_SPEC;
impl crate::RegisterSpec for MACMDIOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmdioar::R](R) reader structure"]
impl crate::Readable for MACMDIOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmdioar::W](W) writer structure"]
impl crate::Writable for MACMDIOAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMDIOAR to value 0"]
impl crate::Resettable for MACMDIOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
