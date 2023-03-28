#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSWU` reader - Master Timer Software update"]
pub type MSWU_R = crate::BitReader<bool>;
#[doc = "Field `MSWU` writer - Master Timer Software update"]
pub type MSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TASWU` reader - Timer A Software update"]
pub type TASWU_R = crate::BitReader<bool>;
#[doc = "Field `TASWU` writer - Timer A Software update"]
pub type TASWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TBSWU` reader - Timer B Software Update"]
pub type TBSWU_R = crate::BitReader<bool>;
#[doc = "Field `TBSWU` writer - Timer B Software Update"]
pub type TBSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TCSWU` reader - Timer C Software Update"]
pub type TCSWU_R = crate::BitReader<bool>;
#[doc = "Field `TCSWU` writer - Timer C Software Update"]
pub type TCSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TDSWU` reader - Timer D Software Update"]
pub type TDSWU_R = crate::BitReader<bool>;
#[doc = "Field `TDSWU` writer - Timer D Software Update"]
pub type TDSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TESWU` reader - Timer E Software Update"]
pub type TESWU_R = crate::BitReader<bool>;
#[doc = "Field `TESWU` writer - Timer E Software Update"]
pub type TESWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TFSWU` reader - Timer f Software Update"]
pub type TFSWU_R = crate::BitReader<bool>;
#[doc = "Field `TFSWU` writer - Timer f Software Update"]
pub type TFSWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `MRST` reader - Master Counter software reset"]
pub type MRST_R = crate::BitReader<bool>;
#[doc = "Field `MRST` writer - Master Counter software reset"]
pub type MRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TARST` reader - Timer A counter software reset"]
pub type TARST_R = crate::BitReader<bool>;
#[doc = "Field `TARST` writer - Timer A counter software reset"]
pub type TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TBRST` reader - Timer B counter software reset"]
pub type TBRST_R = crate::BitReader<bool>;
#[doc = "Field `TBRST` writer - Timer B counter software reset"]
pub type TBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TCRST` reader - Timer C counter software reset"]
pub type TCRST_R = crate::BitReader<bool>;
#[doc = "Field `TCRST` writer - Timer C counter software reset"]
pub type TCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TDRST` reader - Timer D counter software reset"]
pub type TDRST_R = crate::BitReader<bool>;
#[doc = "Field `TDRST` writer - Timer D counter software reset"]
pub type TDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TERST` reader - Timer E counter software reset"]
pub type TERST_R = crate::BitReader<bool>;
#[doc = "Field `TERST` writer - Timer E counter software reset"]
pub type TERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TFRST` reader - Timer f counter software reset"]
pub type TFRST_R = crate::BitReader<bool>;
#[doc = "Field `TFRST` writer - Timer f counter software reset"]
pub type TFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWPA` reader - Swap Timer A outputs"]
pub type SWPA_R = crate::BitReader<bool>;
#[doc = "Field `SWPA` writer - Swap Timer A outputs"]
pub type SWPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWPB` reader - Swap Timer B outputs"]
pub type SWPB_R = crate::BitReader<bool>;
#[doc = "Field `SWPB` writer - Swap Timer B outputs"]
pub type SWPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWPC` reader - Swap Timer C outputs"]
pub type SWPC_R = crate::BitReader<bool>;
#[doc = "Field `SWPC` writer - Swap Timer C outputs"]
pub type SWPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWPD` reader - Swap Timer D outputs"]
pub type SWPD_R = crate::BitReader<bool>;
#[doc = "Field `SWPD` writer - Swap Timer D outputs"]
pub type SWPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWPE` reader - Swap Timer E outputs"]
pub type SWPE_R = crate::BitReader<bool>;
#[doc = "Field `SWPE` writer - Swap Timer E outputs"]
pub type SWPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWPF` reader - Swap Timer F outputs"]
pub type SWPF_R = crate::BitReader<bool>;
#[doc = "Field `SWPF` writer - Swap Timer F outputs"]
pub type SWPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&self) -> MSWU_R {
        MSWU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&self) -> TASWU_R {
        TASWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&self) -> TBSWU_R {
        TBSWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&self) -> TCSWU_R {
        TCSWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&self) -> TDSWU_R {
        TDSWU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&self) -> TESWU_R {
        TESWU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer f Software Update"]
    #[inline(always)]
    pub fn tfswu(&self) -> TFSWU_R {
        TFSWU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timer f counter software reset"]
    #[inline(always)]
    pub fn tfrst(&self) -> TFRST_R {
        TFRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Swap Timer A outputs"]
    #[inline(always)]
    pub fn swpa(&self) -> SWPA_R {
        SWPA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Swap Timer B outputs"]
    #[inline(always)]
    pub fn swpb(&self) -> SWPB_R {
        SWPB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Swap Timer C outputs"]
    #[inline(always)]
    pub fn swpc(&self) -> SWPC_R {
        SWPC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Swap Timer D outputs"]
    #[inline(always)]
    pub fn swpd(&self) -> SWPD_R {
        SWPD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Swap Timer E outputs"]
    #[inline(always)]
    pub fn swpe(&self) -> SWPE_R {
        SWPE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Swap Timer F outputs"]
    #[inline(always)]
    pub fn swpf(&self) -> SWPF_R {
        SWPF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    #[must_use]
    pub fn mswu(&mut self) -> MSWU_W<0> {
        MSWU_W::new(self)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    #[must_use]
    pub fn taswu(&mut self) -> TASWU_W<1> {
        TASWU_W::new(self)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tbswu(&mut self) -> TBSWU_W<2> {
        TBSWU_W::new(self)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tcswu(&mut self) -> TCSWU_W<3> {
        TCSWU_W::new(self)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tdswu(&mut self) -> TDSWU_W<4> {
        TDSWU_W::new(self)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn teswu(&mut self) -> TESWU_W<5> {
        TESWU_W::new(self)
    }
    #[doc = "Bit 6 - Timer f Software Update"]
    #[inline(always)]
    #[must_use]
    pub fn tfswu(&mut self) -> TFSWU_W<6> {
        TFSWU_W::new(self)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn mrst(&mut self) -> MRST_W<8> {
        MRST_W::new(self)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tarst(&mut self) -> TARST_W<9> {
        TARST_W::new(self)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tbrst(&mut self) -> TBRST_W<10> {
        TBRST_W::new(self)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcrst(&mut self) -> TCRST_W<11> {
        TCRST_W::new(self)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tdrst(&mut self) -> TDRST_W<12> {
        TDRST_W::new(self)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn terst(&mut self) -> TERST_W<13> {
        TERST_W::new(self)
    }
    #[doc = "Bit 14 - Timer f counter software reset"]
    #[inline(always)]
    #[must_use]
    pub fn tfrst(&mut self) -> TFRST_W<14> {
        TFRST_W::new(self)
    }
    #[doc = "Bit 16 - Swap Timer A outputs"]
    #[inline(always)]
    #[must_use]
    pub fn swpa(&mut self) -> SWPA_W<16> {
        SWPA_W::new(self)
    }
    #[doc = "Bit 17 - Swap Timer B outputs"]
    #[inline(always)]
    #[must_use]
    pub fn swpb(&mut self) -> SWPB_W<17> {
        SWPB_W::new(self)
    }
    #[doc = "Bit 18 - Swap Timer C outputs"]
    #[inline(always)]
    #[must_use]
    pub fn swpc(&mut self) -> SWPC_W<18> {
        SWPC_W::new(self)
    }
    #[doc = "Bit 19 - Swap Timer D outputs"]
    #[inline(always)]
    #[must_use]
    pub fn swpd(&mut self) -> SWPD_W<19> {
        SWPD_W::new(self)
    }
    #[doc = "Bit 20 - Swap Timer E outputs"]
    #[inline(always)]
    #[must_use]
    pub fn swpe(&mut self) -> SWPE_W<20> {
        SWPE_W::new(self)
    }
    #[doc = "Bit 21 - Swap Timer F outputs"]
    #[inline(always)]
    #[must_use]
    pub fn swpf(&mut self) -> SWPF_W<21> {
        SWPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
