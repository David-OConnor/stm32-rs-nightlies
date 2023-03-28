#[doc = "Register `BMTRG` reader"]
pub struct R(crate::R<BMTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMTRG` writer"]
pub struct W(crate::W<BMTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMTRG_SPEC>;
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
impl From<crate::W<BMTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - SW"]
pub type SW_R = crate::BitReader<bool>;
#[doc = "Field `SW` writer - SW"]
pub type SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `MSTRST` reader - MSTRST"]
pub type MSTRST_R = crate::BitReader<bool>;
#[doc = "Field `MSTRST` writer - MSTRST"]
pub type MSTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `MSTREP` reader - MSTREP"]
pub type MSTREP_R = crate::BitReader<bool>;
#[doc = "Field `MSTREP` writer - MSTREP"]
pub type MSTREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub type MSTCMP1_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `MSTCMP2` reader - MSTCMP2"]
pub type MSTCMP2_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP2` writer - MSTCMP2"]
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `MSTCMP3` reader - MSTCMP3"]
pub type MSTCMP3_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP3` writer - MSTCMP3"]
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `MSTCMP4` reader - MSTCMP4"]
pub type MSTCMP4_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP4` writer - MSTCMP4"]
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TARST` reader - TARST"]
pub type TARST_R = crate::BitReader<bool>;
#[doc = "Field `TARST` writer - TARST"]
pub type TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TAREP` reader - TAREP"]
pub type TAREP_R = crate::BitReader<bool>;
#[doc = "Field `TAREP` writer - TAREP"]
pub type TAREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TACMP1` reader - TACMP1"]
pub type TACMP1_R = crate::BitReader<bool>;
#[doc = "Field `TACMP1` writer - TACMP1"]
pub type TACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TACMP2` reader - TACMP2"]
pub type TACMP2_R = crate::BitReader<bool>;
#[doc = "Field `TACMP2` writer - TACMP2"]
pub type TACMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TBRST` reader - TBRST"]
pub type TBRST_R = crate::BitReader<bool>;
#[doc = "Field `TBRST` writer - TBRST"]
pub type TBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TBREP` reader - TBREP"]
pub type TBREP_R = crate::BitReader<bool>;
#[doc = "Field `TBREP` writer - TBREP"]
pub type TBREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TBCMP1` reader - TBCMP1"]
pub type TBCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TBCMP1` writer - TBCMP1"]
pub type TBCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TBCMP2` reader - TBCMP2"]
pub type TBCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TBCMP2` writer - TBCMP2"]
pub type TBCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TCRST` reader - TCRST"]
pub type TCRST_R = crate::BitReader<bool>;
#[doc = "Field `TCRST` writer - TCRST"]
pub type TCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TCREP` reader - TCREP"]
pub type TCREP_R = crate::BitReader<bool>;
#[doc = "Field `TCREP` writer - TCREP"]
pub type TCREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TCCMP1` reader - TCCMP1"]
pub type TCCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TCCMP1` writer - TCCMP1"]
pub type TCCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TCCMP2` reader - TCCMP2"]
pub type TCCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TCCMP2` writer - TCCMP2"]
pub type TCCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TDRST` reader - TDRST"]
pub type TDRST_R = crate::BitReader<bool>;
#[doc = "Field `TDRST` writer - TDRST"]
pub type TDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TDREP` reader - TDREP"]
pub type TDREP_R = crate::BitReader<bool>;
#[doc = "Field `TDREP` writer - TDREP"]
pub type TDREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TDCMP1` reader - TDCMP1"]
pub type TDCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TDCMP1` writer - TDCMP1"]
pub type TDCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TDCMP2` reader - TDCMP2"]
pub type TDCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TDCMP2` writer - TDCMP2"]
pub type TDCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TERST` reader - TERST"]
pub type TERST_R = crate::BitReader<bool>;
#[doc = "Field `TERST` writer - TERST"]
pub type TERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TEREP` reader - TEREP"]
pub type TEREP_R = crate::BitReader<bool>;
#[doc = "Field `TEREP` writer - TEREP"]
pub type TEREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TECMP1` reader - TECMP1"]
pub type TECMP1_R = crate::BitReader<bool>;
#[doc = "Field `TECMP1` writer - TECMP1"]
pub type TECMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `TECMP2` reader - TECMP2"]
pub type TECMP2_R = crate::BitReader<bool>;
#[doc = "Field `TECMP2` writer - TECMP2"]
pub type TECMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
#[doc = "Field `OCHPEV` reader - OCHPEV"]
pub type OCHPEV_R = crate::BitReader<bool>;
#[doc = "Field `OCHPEV` writer - OCHPEV"]
pub type OCHPEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMTRG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    #[must_use]
    pub fn mstrst(&mut self) -> MSTRST_W<1> {
        MSTRST_W::new(self)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    #[must_use]
    pub fn mstrep(&mut self) -> MSTREP_W<2> {
        MSTREP_W::new(self)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<3> {
        MSTCMP1_W::new(self)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<4> {
        MSTCMP2_W::new(self)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<5> {
        MSTCMP3_W::new(self)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    #[must_use]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<6> {
        MSTCMP4_W::new(self)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<7> {
        TARST_W::new(self)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    #[must_use]
    pub fn tarep(&mut self) -> TAREP_W<8> {
        TAREP_W::new(self)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp1(&mut self) -> TACMP1_W<9> {
        TACMP1_W::new(self)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tacmp2(&mut self) -> TACMP2_W<10> {
        TACMP2_W::new(self)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<11> {
        TBRST_W::new(self)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    #[must_use]
    pub fn tbrep(&mut self) -> TBREP_W<12> {
        TBREP_W::new(self)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp1(&mut self) -> TBCMP1_W<13> {
        TBCMP1_W::new(self)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tbcmp2(&mut self) -> TBCMP2_W<14> {
        TBCMP2_W::new(self)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<15> {
        TCRST_W::new(self)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    #[must_use]
    pub fn tcrep(&mut self) -> TCREP_W<16> {
        TCREP_W::new(self)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp1(&mut self) -> TCCMP1_W<17> {
        TCCMP1_W::new(self)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tccmp2(&mut self) -> TCCMP2_W<18> {
        TCCMP2_W::new(self)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<19> {
        TDRST_W::new(self)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    #[must_use]
    pub fn tdrep(&mut self) -> TDREP_W<20> {
        TDREP_W::new(self)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp1(&mut self) -> TDCMP1_W<21> {
        TDCMP1_W::new(self)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tdcmp2(&mut self) -> TDCMP2_W<22> {
        TDCMP2_W::new(self)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<23> {
        TERST_W::new(self)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    #[must_use]
    pub fn terep(&mut self) -> TEREP_W<24> {
        TEREP_W::new(self)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp1(&mut self) -> TECMP1_W<25> {
        TECMP1_W::new(self)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    #[must_use]
    pub fn tecmp2(&mut self) -> TECMP2_W<26> {
        TECMP2_W::new(self)
    }
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    #[must_use]
    pub fn ochpev(&mut self) -> OCHPEV_W<31> {
        OCHPEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMTRG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmtrg](index.html) module"]
pub struct BMTRG_SPEC;
impl crate::RegisterSpec for BMTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmtrg::R](R) reader structure"]
impl crate::Readable for BMTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmtrg::W](W) writer structure"]
impl crate::Writable for BMTRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMTRG to value 0"]
impl crate::Resettable for BMTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
