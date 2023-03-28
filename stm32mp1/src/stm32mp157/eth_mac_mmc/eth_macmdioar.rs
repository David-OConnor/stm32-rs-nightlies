#[doc = "Register `ETH_MACMDIOAR` reader"]
pub struct R(crate::R<ETH_MACMDIOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACMDIOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACMDIOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACMDIOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACMDIOAR` writer"]
pub struct W(crate::W<ETH_MACMDIOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACMDIOAR_SPEC>;
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
impl From<crate::W<ETH_MACMDIOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACMDIOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GB` reader - GB"]
pub type GB_R = crate::BitReader<bool>;
#[doc = "Field `GB` writer - GB"]
pub type GB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `C45E` reader - C45E"]
pub type C45E_R = crate::BitReader<bool>;
#[doc = "Field `C45E` writer - C45E"]
pub type C45E_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `GOC` reader - GOC"]
pub type GOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GOC` writer - GOC"]
pub type GOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SKAP` reader - SKAP"]
pub type SKAP_R = crate::BitReader<bool>;
#[doc = "Field `SKAP` writer - SKAP"]
pub type SKAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `CR` reader - CR"]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - CR"]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `NTC` reader - NTC"]
pub type NTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NTC` writer - NTC"]
pub type NTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RDA` reader - RDA"]
pub type RDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDA` writer - RDA"]
pub type RDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - PA"]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - PA"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIOAR_SPEC, u8, u8, 5, O>;
#[doc = "Field `BTB` reader - BTB"]
pub type BTB_R = crate::BitReader<bool>;
#[doc = "Field `BTB` writer - BTB"]
pub type BTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
#[doc = "Field `PSE` reader - PSE"]
pub type PSE_R = crate::BitReader<bool>;
#[doc = "Field `PSE` writer - PSE"]
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACMDIOAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GB"]
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GB"]
    #[inline(always)]
    #[must_use]
    pub fn gb(&mut self) -> GB_W<0> {
        GB_W::new(self)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<1> {
        C45E_W::new(self)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<2> {
        GOC_W::new(self)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<4> {
        SKAP_W::new(self)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<8> {
        CR_W::new(self)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<12> {
        NTC_W::new(self)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<16> {
        RDA_W::new(self)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<21> {
        PA_W::new(self)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<26> {
        BTB_W::new(self)
    }
    #[doc = "Bit 27 - PSE"]
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
#[doc = "The MDIO Address register controls the management cycles to external PHY through a management interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdioar](index.html) module"]
pub struct ETH_MACMDIOAR_SPEC;
impl crate::RegisterSpec for ETH_MACMDIOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macmdioar::R](R) reader structure"]
impl crate::Readable for ETH_MACMDIOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macmdioar::W](W) writer structure"]
impl crate::Writable for ETH_MACMDIOAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACMDIOAR to value 0"]
impl crate::Resettable for ETH_MACMDIOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
