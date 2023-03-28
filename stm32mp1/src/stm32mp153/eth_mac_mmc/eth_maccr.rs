#[doc = "Register `ETH_MACCR` reader"]
pub struct R(crate::R<ETH_MACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACCR` writer"]
pub struct W(crate::W<ETH_MACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACCR_SPEC>;
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
impl From<crate::W<ETH_MACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - RE"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - RE"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `TE` reader - TE"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - TE"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `PRELEN` reader - PRELEN"]
pub type PRELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRELEN` writer - PRELEN"]
pub type PRELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC` reader - DC"]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - DC"]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `BL` reader - BL"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - BL"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DR` reader - DR"]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `DR` writer - DR"]
pub type DR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `DCRS` reader - DCRS"]
pub type DCRS_R = crate::BitReader<bool>;
#[doc = "Field `DCRS` writer - DCRS"]
pub type DCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `DO` reader - DO"]
pub type DO_R = crate::BitReader<bool>;
#[doc = "Field `DO` writer - DO"]
pub type DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `ECRSFD` reader - ECRSFD"]
pub type ECRSFD_R = crate::BitReader<bool>;
#[doc = "Field `ECRSFD` writer - ECRSFD"]
pub type ECRSFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `LM` reader - LM"]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - LM"]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `DM` reader - DM"]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - DM"]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `FES` reader - FES"]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - FES"]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `PS` reader - PS"]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - PS"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `JE` reader - JE"]
pub type JE_R = crate::BitReader<bool>;
#[doc = "Field `JE` writer - JE"]
pub type JE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `JD` reader - JD"]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - JD"]
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `BE` reader - BE"]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - BE"]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `WD` reader - WD"]
pub type WD_R = crate::BitReader<bool>;
#[doc = "Field `WD` writer - WD"]
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `ACS` reader - ACS"]
pub type ACS_R = crate::BitReader<bool>;
#[doc = "Field `ACS` writer - ACS"]
pub type ACS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `CST` reader - CST"]
pub type CST_R = crate::BitReader<bool>;
#[doc = "Field `CST` writer - CST"]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `S2KP` reader - S2KP"]
pub type S2KP_R = crate::BitReader<bool>;
#[doc = "Field `S2KP` writer - S2KP"]
pub type S2KP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `GPSLCE` reader - GPSLCE"]
pub type GPSLCE_R = crate::BitReader<bool>;
#[doc = "Field `GPSLCE` writer - GPSLCE"]
pub type GPSLCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `IPG` reader - IPG"]
pub type IPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPG` writer - IPG"]
pub type IPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IPC` reader - IPC"]
pub type IPC_R = crate::BitReader<bool>;
#[doc = "Field `IPC` writer - IPC"]
pub type IPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
#[doc = "Field `SARC` reader - SARC"]
pub type SARC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARC` writer - SARC"]
pub type SARC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ARPEN` reader - ARPEN"]
pub type ARPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARPEN` writer - ARPEN"]
pub type ARPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RE"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TE"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PRELEN"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DC"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCRS"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DO"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ECRSFD"]
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LM"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DM"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FES"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PS"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - JE"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - JD"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WD"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ACS"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CST"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - S2KP"]
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPSLCE"]
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - IPG"]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - IPC"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SARC"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RE"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<0> {
        RE_W::new(self)
    }
    #[doc = "Bit 1 - TE"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<1> {
        TE_W::new(self)
    }
    #[doc = "Bits 2:3 - PRELEN"]
    #[inline(always)]
    #[must_use]
    pub fn prelen(&mut self) -> PRELEN_W<2> {
        PRELEN_W::new(self)
    }
    #[doc = "Bit 4 - DC"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<8> {
        DR_W::new(self)
    }
    #[doc = "Bit 9 - DCRS"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DCRS_W<9> {
        DCRS_W::new(self)
    }
    #[doc = "Bit 10 - DO"]
    #[inline(always)]
    #[must_use]
    pub fn do_(&mut self) -> DO_W<10> {
        DO_W::new(self)
    }
    #[doc = "Bit 11 - ECRSFD"]
    #[inline(always)]
    #[must_use]
    pub fn ecrsfd(&mut self) -> ECRSFD_W<11> {
        ECRSFD_W::new(self)
    }
    #[doc = "Bit 12 - LM"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - DM"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<13> {
        DM_W::new(self)
    }
    #[doc = "Bit 14 - FES"]
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    #[doc = "Bit 15 - PS"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<15> {
        PS_W::new(self)
    }
    #[doc = "Bit 16 - JE"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JE_W<16> {
        JE_W::new(self)
    }
    #[doc = "Bit 17 - JD"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<17> {
        JD_W::new(self)
    }
    #[doc = "Bit 18 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<18> {
        BE_W::new(self)
    }
    #[doc = "Bit 19 - WD"]
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<19> {
        WD_W::new(self)
    }
    #[doc = "Bit 20 - ACS"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> ACS_W<20> {
        ACS_W::new(self)
    }
    #[doc = "Bit 21 - CST"]
    #[inline(always)]
    #[must_use]
    pub fn cst(&mut self) -> CST_W<21> {
        CST_W::new(self)
    }
    #[doc = "Bit 22 - S2KP"]
    #[inline(always)]
    #[must_use]
    pub fn s2kp(&mut self) -> S2KP_W<22> {
        S2KP_W::new(self)
    }
    #[doc = "Bit 23 - GPSLCE"]
    #[inline(always)]
    #[must_use]
    pub fn gpslce(&mut self) -> GPSLCE_W<23> {
        GPSLCE_W::new(self)
    }
    #[doc = "Bits 24:26 - IPG"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IPG_W<24> {
        IPG_W::new(self)
    }
    #[doc = "Bit 27 - IPC"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IPC_W<27> {
        IPC_W::new(self)
    }
    #[doc = "Bits 28:30 - SARC"]
    #[inline(always)]
    #[must_use]
    pub fn sarc(&mut self) -> SARC_W<28> {
        SARC_W::new(self)
    }
    #[doc = "Bit 31 - ARPEN"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<31> {
        ARPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MAC Configuration Register establishes the operating mode of the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maccr](index.html) module"]
pub struct ETH_MACCR_SPEC;
impl crate::RegisterSpec for ETH_MACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maccr::R](R) reader structure"]
impl crate::Readable for ETH_MACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maccr::W](W) writer structure"]
impl crate::Writable for ETH_MACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACCR to value 0x8000"]
impl crate::Resettable for ETH_MACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
