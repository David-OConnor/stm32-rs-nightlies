#[doc = "Register `DDRPHYC_DDR3_MR1` reader"]
pub struct R(crate::R<DDRPHYC_DDR3_MR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DDR3_MR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DDR3_MR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DDR3_MR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DDR3_MR1` writer"]
pub struct W(crate::W<DDRPHYC_DDR3_MR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DDR3_MR1_SPEC>;
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
impl From<crate::W<DDRPHYC_DDR3_MR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DDR3_MR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DE` reader - DE"]
pub type DE_R = crate::BitReader<bool>;
#[doc = "Field `DE` writer - DE"]
pub type DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `DIC0` reader - DIC0"]
pub type DIC0_R = crate::BitReader<bool>;
#[doc = "Field `DIC0` writer - DIC0"]
pub type DIC0_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `RTT0` reader - RTT0"]
pub type RTT0_R = crate::BitReader<bool>;
#[doc = "Field `RTT0` writer - RTT0"]
pub type RTT0_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `AL` reader - AL"]
pub type AL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AL` writer - AL"]
pub type AL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIC1` reader - DIC1"]
pub type DIC1_R = crate::BitReader<bool>;
#[doc = "Field `DIC1` writer - DIC1"]
pub type DIC1_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `RTT1` reader - RTT1"]
pub type RTT1_R = crate::BitReader<bool>;
#[doc = "Field `RTT1` writer - RTT1"]
pub type RTT1_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `LEVEL` reader - LEVEL"]
pub type LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `LEVEL` writer - LEVEL"]
pub type LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `RTT2` reader - RTT2"]
pub type RTT2_R = crate::BitReader<bool>;
#[doc = "Field `RTT2` writer - RTT2"]
pub type RTT2_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `TDQS` reader - TDQS"]
pub type TDQS_R = crate::BitReader<bool>;
#[doc = "Field `TDQS` writer - TDQS"]
pub type TDQS_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
#[doc = "Field `QOFF` reader - QOFF"]
pub type QOFF_R = crate::BitReader<bool>;
#[doc = "Field `QOFF` writer - QOFF"]
pub type QOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, DDRPHYC_DDR3_MR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    pub fn dic0(&self) -> DIC0_R {
        DIC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    pub fn rtt0(&self) -> RTT0_R {
        RTT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    pub fn dic1(&self) -> DIC1_R {
        DIC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    pub fn rtt1(&self) -> RTT1_R {
        RTT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    pub fn rtt2(&self) -> RTT2_R {
        RTT2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    pub fn tdqs(&self) -> TDQS_R {
        TDQS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    pub fn qoff(&self) -> QOFF_R {
        QOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DE"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<0> {
        DE_W::new(self)
    }
    #[doc = "Bit 1 - DIC0"]
    #[inline(always)]
    #[must_use]
    pub fn dic0(&mut self) -> DIC0_W<1> {
        DIC0_W::new(self)
    }
    #[doc = "Bit 2 - RTT0"]
    #[inline(always)]
    #[must_use]
    pub fn rtt0(&mut self) -> RTT0_W<2> {
        RTT0_W::new(self)
    }
    #[doc = "Bits 3:4 - AL"]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AL_W<3> {
        AL_W::new(self)
    }
    #[doc = "Bit 5 - DIC1"]
    #[inline(always)]
    #[must_use]
    pub fn dic1(&mut self) -> DIC1_W<5> {
        DIC1_W::new(self)
    }
    #[doc = "Bit 6 - RTT1"]
    #[inline(always)]
    #[must_use]
    pub fn rtt1(&mut self) -> RTT1_W<6> {
        RTT1_W::new(self)
    }
    #[doc = "Bit 7 - LEVEL"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<7> {
        LEVEL_W::new(self)
    }
    #[doc = "Bit 9 - RTT2"]
    #[inline(always)]
    #[must_use]
    pub fn rtt2(&mut self) -> RTT2_W<9> {
        RTT2_W::new(self)
    }
    #[doc = "Bit 11 - TDQS"]
    #[inline(always)]
    #[must_use]
    pub fn tdqs(&mut self) -> TDQS_W<11> {
        TDQS_W::new(self)
    }
    #[doc = "Bit 12 - QOFF"]
    #[inline(always)]
    #[must_use]
    pub fn qoff(&mut self) -> QOFF_W<12> {
        QOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC MR1 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ddr3_mr1](index.html) module"]
pub struct DDRPHYC_DDR3_MR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DDR3_MR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ddrphyc_ddr3_mr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DDR3_MR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ddr3_mr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DDR3_MR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DDR3_MR1 to value 0"]
impl crate::Resettable for DDRPHYC_DDR3_MR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
