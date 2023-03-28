#[doc = "Register `FDCAN_TTOCF` reader"]
pub struct R(crate::R<FDCAN_TTOCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTOCF` writer"]
pub struct W(crate::W<FDCAN_TTOCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTOCF_SPEC>;
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
impl From<crate::W<FDCAN_TTOCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTOCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OM` reader - OM"]
pub type OM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OM` writer - OM"]
pub type OM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 2, O>;
#[doc = "Field `GEN` reader - GEN"]
pub type GEN_R = crate::BitReader<bool>;
#[doc = "Field `GEN` writer - GEN"]
pub type GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
#[doc = "Field `TM` reader - TM"]
pub type TM_R = crate::BitReader<bool>;
#[doc = "Field `TM` writer - TM"]
pub type TM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
#[doc = "Field `LDSDL` reader - LDSDL"]
pub type LDSDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDSDL` writer - LDSDL"]
pub type LDSDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 3, O>;
#[doc = "Field `IRTO` reader - IRTO"]
pub type IRTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRTO` writer - IRTO"]
pub type IRTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 7, O>;
#[doc = "Field `EECS` reader - EECS"]
pub type EECS_R = crate::BitReader<bool>;
#[doc = "Field `EECS` writer - EECS"]
pub type EECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
#[doc = "Field `AWL` reader - AWL"]
pub type AWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWL` writer - AWL"]
pub type AWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCF_SPEC, u8, u8, 8, O>;
#[doc = "Field `EGTF` reader - EGTF"]
pub type EGTF_R = crate::BitReader<bool>;
#[doc = "Field `EGTF` writer - EGTF"]
pub type EGTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
#[doc = "Field `ECC` reader - ECC"]
pub type ECC_R = crate::BitReader<bool>;
#[doc = "Field `ECC` writer - ECC"]
pub type ECC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
#[doc = "Field `EVTP` reader - EVTP"]
pub type EVTP_R = crate::BitReader<bool>;
#[doc = "Field `EVTP` writer - EVTP"]
pub type EVTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - OM"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - GEN"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - LDSDL"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - IRTO"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - EECS"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - AWL"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - EGTF"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ECC"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EVTP"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - OM"]
    #[inline(always)]
    #[must_use]
    pub fn om(&mut self) -> OM_W<0> {
        OM_W::new(self)
    }
    #[doc = "Bit 3 - GEN"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<3> {
        GEN_W::new(self)
    }
    #[doc = "Bit 4 - TM"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<4> {
        TM_W::new(self)
    }
    #[doc = "Bits 5:7 - LDSDL"]
    #[inline(always)]
    #[must_use]
    pub fn ldsdl(&mut self) -> LDSDL_W<5> {
        LDSDL_W::new(self)
    }
    #[doc = "Bits 8:14 - IRTO"]
    #[inline(always)]
    #[must_use]
    pub fn irto(&mut self) -> IRTO_W<8> {
        IRTO_W::new(self)
    }
    #[doc = "Bit 15 - EECS"]
    #[inline(always)]
    #[must_use]
    pub fn eecs(&mut self) -> EECS_W<15> {
        EECS_W::new(self)
    }
    #[doc = "Bits 16:23 - AWL"]
    #[inline(always)]
    #[must_use]
    pub fn awl(&mut self) -> AWL_W<16> {
        AWL_W::new(self)
    }
    #[doc = "Bit 24 - EGTF"]
    #[inline(always)]
    #[must_use]
    pub fn egtf(&mut self) -> EGTF_W<24> {
        EGTF_W::new(self)
    }
    #[doc = "Bit 25 - ECC"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<25> {
        ECC_W::new(self)
    }
    #[doc = "Bit 26 - EVTP"]
    #[inline(always)]
    #[must_use]
    pub fn evtp(&mut self) -> EVTP_W<26> {
        EVTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT operation configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttocf](index.html) module"]
pub struct FDCAN_TTOCF_SPEC;
impl crate::RegisterSpec for FDCAN_TTOCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttocf::R](R) reader structure"]
impl crate::Readable for FDCAN_TTOCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttocf::W](W) writer structure"]
impl crate::Writable for FDCAN_TTOCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTOCF to value 0x0001_0000"]
impl crate::Resettable for FDCAN_TTOCF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
